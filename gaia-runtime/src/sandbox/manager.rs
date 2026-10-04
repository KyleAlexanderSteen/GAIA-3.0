//! [`SandboxManager`] — Wasmtime 46 + WASI 0.3 (p3) real isolation backend.
//!
//! # Store data layout
//!
//! `wasmtime_wasi::p3::add_to_linker` requires `T: WasiView`.
//! In wasmtime-wasi 46 `WasiView` has a single required method:
//!
//! ```text
//! fn ctx(&mut self) -> WasiCtxView<'_>
//! ```
//!
//! `WasiCtxView` is a struct literal with two fields:
//!   `{ ctx: &mut WasiCtx, table: &mut ResourceTable }`
//! where `ResourceTable` lives in `wasmtime::component`.
//!
//! `StoreData` bundles all three pieces so there is no unsafe memory leaking.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use wasmtime::{
    component::{
        Component,
        Linker,
        ResourceTable,
    },
    Config, Engine, Store,
};
use wasmtime_wasi::{
    DirPerms, FilePerms,
    WasiCtx, WasiCtxBuilder, WasiCtxView,
    WasiView,
};

use crate::load_recovery::{LoadAssessment, LoadThresholds, RuntimeLoadTelemetry};

use crate::sandbox::{
    error::{SandboxError, GAIA_CAPABILITY_DENIED},
    limits::GaiaResourceLimiter,
    profile::SandboxProfile,
};

// ── StoreData ────────────────────────────────────────────────────────────────

/// Data stored inside every Wasmtime `Store` created by this manager.
pub struct StoreData {
    pub wasi:    WasiCtx,
    pub table:   ResourceTable,
    pub limiter: GaiaResourceLimiter,
}

impl WasiView for StoreData {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx:   &mut self.wasi,
            table: &mut self.table,
        }
    }
}

// ── SandboxManager ───────────────────────────────────────────────────────────

pub struct SandboxManager {
    engine:  Engine,
    profile: SandboxProfile,
}

impl SandboxManager {
    pub fn new(profile: SandboxProfile) -> Result<Self, SandboxError> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        config.epoch_interruption(true);

        let engine = Engine::new(&config)
            .map_err(|e| SandboxError::EngineInit(anyhow::Error::from(e)))?;
        Ok(Self { engine, profile })
    }

    pub fn build_wasi_ctx(&self) -> Result<WasiCtx, SandboxError> {
        let mut builder = WasiCtxBuilder::new();

        if self.profile.scratch_only_writes {
            let scratch = scratch_dir();
            std::fs::create_dir_all(&scratch).map_err(|e| {
                SandboxError::CapabilityDenied(format!(
                    "{GAIA_CAPABILITY_DENIED}: cannot create scratch dir: {e}"
                ))
            })?;
            builder
                .preopened_dir(&scratch, "/scratch", DirPerms::all(), FilePerms::all())
                .map_err(|e| {
                    SandboxError::CapabilityDenied(format!(
                        "{GAIA_CAPABILITY_DENIED}: preopened_dir failed: {e}"
                    ))
                })?;
        }

        if self.profile.inherited_env {
            builder.inherit_env();
        }

        Ok(builder.build())
    }

    pub fn compile(&self, bytes: &[u8]) -> Result<Component, SandboxError> {
        Component::new(&self.engine, bytes)
            .map_err(|e| SandboxError::CompileError(e.to_string()))
    }

    /// Execute a component while collecting the runtime/resource measurements
    /// required by the #1691 load/recovery contract.
    ///
    /// The caller supplies explicit budgets because the runtime cannot infer
    /// universal workload limits. The returned telemetry is observational; it
    /// does not itself authorize pause/stop/recovery actions.
    pub async fn execute_component_with_load(
        &self,
        component: &Component,
        duration_budget: Duration,
        cpu_millis_budget: u64,
    ) -> (Result<(), SandboxError>, RuntimeLoadTelemetry) {
        let started = Instant::now();
        let wasi = match self.build_wasi_ctx() {
            Ok(wasi) => wasi,
            Err(error) => {
                return (
                    Err(error),
                    RuntimeLoadTelemetry {
                        elapsed: started.elapsed(),
                        duration_budget,
                        cpu_millis_used: cpu_millis_budget,
                        cpu_millis_budget,
                        memory_bytes_used: 0,
                        memory_bytes_budget: self.profile.quota.max_memory_bytes,
                        operations: 1,
                        failures: 1,
                        contradictions: 0,
                        interruptions: 0,
                        explicit_stop: false,
                    },
                );
            }
        };

        let memory_budget = self.profile.quota.max_memory_bytes;
        let store_data = StoreData {
            wasi,
            table: ResourceTable::new(),
            limiter: GaiaResourceLimiter::new(self.profile.quota),
        };
        let mut store = Store::new(&self.engine, store_data);
        store.limiter(|data: &mut StoreData| &mut data.limiter);
        store.set_epoch_deadline(self.profile.quota.max_epochs);

        let mut linker: Linker<StoreData> = Linker::new(&self.engine);
        let result = async {
            wasmtime_wasi::p3::add_to_linker(&mut linker)
                .map_err(|e| SandboxError::EngineInit(anyhow::Error::from(e)))?;
            let instance = linker
                .instantiate_async(&mut store, component)
                .await
                .map_err(|e| Self::classify_trap(&e.to_string()))?;
            if let Some(f) = instance
                .get_func(&mut store, "run")
                .or_else(|| instance.get_func(&mut store, "wasi:cli/run@0.3.0#run"))
            {
                f.call_async(&mut store, &[], &mut [])
                    .await
                    .map(|_| ())
                    .map_err(|e| Self::classify_trap(&e.to_string()))?;
            }
            Ok::<(), SandboxError>(())
        }
        .await;

        let memory_used = store.data().limiter.mem_used();
        let elapsed = started.elapsed();
        let failed = result.is_err();
        let interruptions = u64::from(matches!(&result, Err(SandboxError::Timeout)));
        let telemetry = RuntimeLoadTelemetry {
            elapsed,
            duration_budget,
            cpu_millis_used: if failed { cpu_millis_budget } else { elapsed.as_millis() as u64 },
            cpu_millis_budget,
            memory_bytes_used: memory_used,
            memory_bytes_budget: memory_budget,
            operations: 1,
            failures: u64::from(failed),
            contradictions: 0,
            interruptions,
            explicit_stop: false,
        };
        (result, telemetry)
    }

    pub fn assess_load(&self, telemetry: RuntimeLoadTelemetry, thresholds: LoadThresholds) -> LoadAssessment {
        telemetry.assess(thresholds)
    }

    pub async fn execute_component(
        &self,
        component: &Component,
    ) -> Result<(), SandboxError> {
        let wasi = self.build_wasi_ctx()?;

        let store_data = StoreData {
            wasi,
            table:   ResourceTable::new(),
            limiter: GaiaResourceLimiter::new(self.profile.quota),
        };
        let mut store = Store::new(&self.engine, store_data);

        store.limiter(|data: &mut StoreData| &mut data.limiter);
        store.set_epoch_deadline(self.profile.quota.max_epochs);

        let mut linker: Linker<StoreData> = Linker::new(&self.engine);

        wasmtime_wasi::p3::add_to_linker(&mut linker)
            .map_err(|e| SandboxError::EngineInit(anyhow::Error::from(e)))?;

        // .to_string() converts wasmtime::Error without requiring StdError impl.
        let instance = linker
            .instantiate_async(&mut store, component)
            .await
            .map_err(|e| Self::classify_trap(&e.to_string()))?;

        let func = instance
            .get_func(&mut store, "run")
            .or_else(|| instance.get_func(&mut store, "wasi:cli/run@0.3.0#run"));

        if let Some(f) = func {
            f.call_async(&mut store, &[], &mut [])
                .await
                .map(|_| ())
                .map_err(|e| Self::classify_trap(&e.to_string()))?;
        }

        Ok(())
    }

    /// Classify an error message string into a [`SandboxError`].
    ///
    /// Accepts `&str` so it is independent of any error type's trait impls.
    /// Production call-sites convert via `.to_string()`. Unit tests do the
    /// same: `SandboxManager::classify_trap(&err.to_string())`.
    pub fn classify_trap(msg: &str) -> SandboxError {
        let lower = msg.to_lowercase();
        if lower.contains("out of memory") || lower.contains("oom") {
            SandboxError::OomTermination
        } else if lower.contains("epoch") || lower.contains("interrupt") {
            SandboxError::Timeout
        } else if lower.contains("eacces") || lower.contains("permission denied") {
            SandboxError::CapabilityDenied(GAIA_CAPABILITY_DENIED.to_owned())
        } else {
            SandboxError::Trap(msg.to_owned())
        }
    }

    pub fn profile(&self) -> &SandboxProfile {
        &self.profile
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }
}

fn scratch_dir() -> PathBuf {
    std::env::temp_dir().join("gaia-sandbox-scratch")
}
