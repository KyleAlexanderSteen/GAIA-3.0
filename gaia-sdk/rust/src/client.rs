use ed25519_dalek::{Keypair, PublicKey, Signature as DalekSig, Signer, Verifier};
use rand::rngs::OsRng;

use crate::error::{GaiaError, Result};
use crate::types::*;

pub struct GaiaClient {
    keypair: Keypair,
}

impl Default for GaiaClient {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for GaiaClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GaiaClient").finish_non_exhaustive()
    }
}

impl GaiaClient {
    pub fn new() -> Self {
        let mut rng = OsRng {};
        Self {
            keypair: Keypair::generate(&mut rng),
        }
    }

    pub fn intent(&self, goal: impl Into<String>) -> Result<TaskHandle> {
        let goal = goal.into();
        if goal.trim().is_empty() {
            return Err(GaiaError::InvalidArgument("goal must not be empty".into()));
        }
        Err(GaiaError::NotImplemented(
            "intent has no kernel transport; nothing was admitted".into(),
        ))
    }

    pub fn context(&self, query: SemanticQuery) -> Result<MemCube> {
        if query.text.trim().is_empty() {
            return Err(GaiaError::InvalidArgument("query must not be empty".into()));
        }
        Err(GaiaError::NotImplemented(
            "context has no MemOS transport; nothing was stored".into(),
        ))
    }

    pub fn invoke(&self, agent: AgentSpec) -> Result<String> {
        if agent.agent_id.is_empty() {
            return Err(GaiaError::InvalidArgument("agent_id required".into()));
        }
        Err(GaiaError::NotImplemented(format!(
            "invoke has no registered agent: {}",
            agent.name
        )))
    }

    pub fn observe(&self, sensor: &str) -> Result<String> {
        if sensor.is_empty() {
            return Err(GaiaError::InvalidArgument("sensor required".into()));
        }
        Err(GaiaError::NotImplemented(format!(
            "observe has no sensor bus: {sensor}"
        )))
    }

    pub fn sign(&self, payload: &[u8]) -> Result<Signature> {
        if payload.is_empty() {
            return Err(GaiaError::InvalidArgument("payload required".into()));
        }
        let sig = self.keypair.sign(payload);
        let mut bytes = self.keypair.public.as_bytes().to_vec();
        bytes.extend_from_slice(&sig.to_bytes());
        Ok(Signature {
            algorithm: "ed25519".into(),
            bytes,
        })
    }

    pub fn verify(&self, payload: &[u8], sig: &Signature) -> Result<bool> {
        if payload.is_empty() {
            return Err(GaiaError::InvalidArgument("payload required".into()));
        }
        if sig.algorithm != "ed25519" || sig.bytes.len() != 32 + 64 {
            return Ok(false);
        }
        let Ok(pk) = PublicKey::from_bytes(&sig.bytes[..32]) else {
            return Ok(false);
        };
        let Ok(ds) = DalekSig::from_bytes(&sig.bytes[32..]) else {
            return Ok(false);
        };
        Ok(pk.verify(payload, &ds).is_ok())
    }

    pub fn declare(&self, resource: ResourceSpec) -> Result<ResourceHandle> {
        if resource.name.is_empty() {
            return Err(GaiaError::InvalidArgument("resource name required".into()));
        }
        Err(GaiaError::NotImplemented(format!(
            "declare has no resource registry: {}",
            resource.name
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intent_is_not_implemented() {
        let err = GaiaClient::new().intent("hello").unwrap_err();
        assert!(matches!(err, GaiaError::NotImplemented(_)));
    }

    #[test]
    fn intent_rejects_empty() {
        let err = GaiaClient::new().intent("  ").unwrap_err();
        assert!(matches!(err, GaiaError::InvalidArgument(_)));
    }

    #[test]
    fn sign_verify_roundtrip() {
        let c = GaiaClient::new();
        let sig = c.sign(b"intent-proof").unwrap();
        assert!(c.verify(b"intent-proof", &sig).unwrap());
        assert!(!c.verify(b"other", &sig).unwrap());
    }
}
