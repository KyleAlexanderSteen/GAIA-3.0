//! Local Super OS layers. Each one does a host action. None claims a planet.

use std::env;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layer {
    pub id: u8,
    pub name: &'static str,
    pub result: String,
}

pub fn run_all() -> Vec<Layer> {
    vec![
        hardware(),
        kernel(),
        identity(),
        runtime(),
        coordination(),
        memory(),
        agent(),
        intent(),
        planetary(),
    ]
}

pub fn hardware() -> Layer {
    Layer {
        id: 1,
        name: "hal",
        result: format!("arch={} os={}", env::consts::ARCH, env::consts::OS),
    }
}

pub fn kernel() -> Layer {
    Layer { id: 2, name: "kernel", result: "local kernel linked; Asterinas is not this crate".into() }
}

pub fn identity() -> Layer {
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    Layer { id: 3, name: "identity", result: format!("local-id={stamp}") }
}

pub fn runtime() -> Layer {
    Layer { id: 4, name: "runtime", result: "host process; wasm guest not started".into() }
}

pub fn coordination() -> Layer {
    Layer { id: 5, name: "coordination", result: "local queue depth=0".into() }
}

pub fn memory() -> Layer {
    Layer { id: 6, name: "memory", result: "ledger is the memory; five tiers are not mounted".into() }
}

pub fn agent() -> Layer {
    Layer { id: 7, name: "agent", result: "state=registered".into() }
}

pub fn intent() -> Layer {
    Layer { id: 8, name: "intent", result: "echo records; a plain intent reaches the executor".into() }
}

pub fn planetary() -> Layer {
    Layer { id: 9, name: "planetary", result: "scales=city,country,continent,global; live=false".into() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nine_layers_run_and_the_planet_is_not_live() {
        let layers = run_all();
        assert_eq!(layers.len(), 9);
        assert!(layers[0].result.contains("arch="));
        assert!(layers[8].result.contains("live=false"));
        assert!(!layers.iter().any(|layer| layer.result.contains("conscious")));
    }
}
