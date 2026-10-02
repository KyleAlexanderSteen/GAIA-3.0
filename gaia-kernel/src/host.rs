use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use gaia_memos::{CubeType, MemCube, MemOs};
use gaia_sfs::{SemanticMeta, Sfs, SfsObject};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::audit::{AuditLog, AuditRecord};
use crate::broker::{Broker, Capabilities};
use crate::executor::Executor;
use crate::identity::{verify, Principal};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum KernelError {
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
    #[error("denied: {0}")]
    Denied(String),
    #[error("not implemented: {0}")]
    NotImplemented(String),
    #[error("sfs: {0}")]
    Sfs(String),
    #[error("memos: {0}")]
    Memos(String),
    /// A capability was registered with fewer than two resolution paths,
    /// giving one node exclusive control (Monopoly Architect adversarial scenario).
    #[error("GAIA_KERNEL_MONOPOLY_RISK: capability has fewer than two resolution paths")]
    MonopolyRisk,
}

pub type Result<T> = std::result::Result<T, KernelError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskHandle {
    pub intent_id: Uuid,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceHandle {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedBlob {
    pub algorithm: String,
    pub public_hex: String,
    pub signature_hex: String,
}

/// Userspace kernel implementing the Phase 1 syscall ABI.
pub struct KernelHost {
    pub principal: Principal,
    pub require_signed: bool,
    pub broker: Arc<Broker>,
    pub executor: Executor,
    pub sfs: Sfs,
    pub memos: MemOs,
    pub audit: AuditLog,
}

impl KernelHost {
    pub fn new(data_dir: PathBuf) -> Result<Self> {
        let principal = Principal::generate(crate::identity::PrincipalKind::Node);
        let broker = Arc::new(Broker::new());
        let executor = Executor::new(&principal, Capabilities::default(), broker.clone());
        let sfs = Sfs::open(data_dir.join("sfs")).map_err(|e| KernelError::Sfs(e.to_string()))?;
        Ok(Self {
            principal,
            require_signed: true,
            broker,
            executor,
            sfs,
            memos: MemOs::new(),
            audit: AuditLog::default(),
        })
    }

    pub fn intent(&mut self, goal: &str) -> Result<TaskHandle> {
        if goal.trim().is_empty() {
            return Err(KernelError::InvalidArgument(
                "goal must not be empty".into(),
            ));
        }
        let handle = TaskHandle {
            intent_id: Uuid::new_v4(),
            state: "admitted".into(),
        };
        self.audit.append(
            &self.principal,
            "intent",
            &format!("{}:{}", handle.intent_id, goal),
        );
        Ok(handle)
    }

    pub fn context(&mut self, query: &str) -> Result<MemCube> {
        if query.trim().is_empty() {
            return Err(KernelError::InvalidArgument(
                "query must not be empty".into(),
            ));
        }
        if let Some((_, cube)) = self.memos.recall(query, 1).into_iter().next() {
            self.audit
                .append(&self.principal, "context", &cube.id.to_string());
            return Ok(cube);
        }
        let cube = MemCube::new(CubeType::Plaintext, query, "context.miss");
        let id = self.memos.put(cube.clone());
        self.audit
            .append(&self.principal, "context", &id.to_string());
        Ok(cube)
    }

    pub fn invoke(&mut self, agent_id: &str, _params: &str) -> Result<String> {
        if agent_id.is_empty() {
            return Err(KernelError::InvalidArgument("agent_id required".into()));
        }
        self.audit.append(&self.principal, "invoke", agent_id);
        Err(KernelError::NotImplemented(format!(
            "invoke has no registered agent: {agent_id}"
        )))
    }

    pub fn observe(&mut self, sensor: &str) -> Result<String> {
        if sensor.is_empty() {
            return Err(KernelError::InvalidArgument("sensor required".into()));
        }
        self.audit.append(&self.principal, "observe", sensor);
        Err(KernelError::NotImplemented(format!(
            "observe has no sensor feed: {sensor}"
        )))
    }

    pub fn sign(&mut self, payload: &[u8]) -> Result<SignedBlob> {
        if payload.is_empty() {
            return Err(KernelError::InvalidArgument("payload required".into()));
        }
        let blob = SignedBlob {
            algorithm: "ed25519".into(),
            public_hex: self.principal.public_hex(),
            signature_hex: hex::encode(self.principal.sign(payload)),
        };
        self.audit
            .append(&self.principal, "sign", &format!("{}", payload.len()));
        Ok(blob)
    }

    pub fn verify(&self, payload: &[u8], blob: &SignedBlob) -> Result<bool> {
        if payload.is_empty() {
            return Err(KernelError::InvalidArgument("payload required".into()));
        }
        if blob.algorithm != "ed25519" {
            return Ok(false);
        }
        let sig = hex::decode(&blob.signature_hex).unwrap_or_default();
        Ok(verify(&blob.public_hex, payload, &sig))
    }

    pub fn declare(&mut self, name: &str, kind: &str) -> Result<ResourceHandle> {
        if name.is_empty() {
            return Err(KernelError::InvalidArgument(
                "resource name required".into(),
            ));
        }
        let handle = ResourceHandle {
            id: Uuid::new_v4(),
            name: name.into(),
        };
        self.audit
            .append(&self.principal, "declare", &format!("{kind}:{name}"));
        Ok(handle)
    }

    pub fn store_file(&mut self, path: &str, data: &[u8], intent: &str) -> Result<SfsObject> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let meta = SemanticMeta {
            who: self.principal.did(),
            when_unix: now,
            context: "phase-1".into(),
            intent: intent.into(),
        };
        let obj = self
            .sfs
            .put(path, data, meta)
            .map_err(|e| KernelError::Sfs(e.to_string()))?;
        self.audit
            .append(&self.principal, "sfs.put", &format!("{}:{}", path, obj.cid));
        Ok(obj)
    }

    pub fn write_cube(&mut self, content: &str) -> Result<MemCube> {
        let cube = MemCube::new(CubeType::Episodic, content, "syscall");
        self.memos.put(cube.clone());
        self.audit
            .append(&self.principal, "memos.put", &cube.id.to_string());
        Ok(cube)
    }

    pub fn reject_unsigned(&self, provided_sig: Option<&SignedBlob>, payload: &[u8]) -> Result<()> {
        if !self.require_signed {
            return Ok(());
        }
        let Some(blob) = provided_sig else {
            return Err(KernelError::Denied(
                "unsigned operations are rejected".into(),
            ));
        };
        if !self.verify(payload, blob)? {
            return Err(KernelError::Denied("bad signature".into()));
        }
        if blob.public_hex != self.principal.public_hex() {
            return Err(KernelError::Denied(
                "foreign key cannot mint system-wide trust".into(),
            ));
        }
        Ok(())
    }

    pub fn last_audit(&self, op: &str, needle: &str) -> Option<AuditRecord> {
        self.audit.prove(op, needle).cloned()
    }
}
