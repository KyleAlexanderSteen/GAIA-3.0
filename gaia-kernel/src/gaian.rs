//! A companion bound to a provider. Unbounded is refused.

pub trait ModelProvider {
    fn complete(&self, prompt: &str) -> Result<String, &'static str>;
}

pub struct MissingLocal;

impl ModelProvider for MissingLocal {
    fn complete(&self, _prompt: &str) -> Result<String, &'static str> {
        Err("local model did not answer")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gaian {
    pub id: String,
    pub tone: String,
    pub autonomy: String,
}

pub fn new(id: &str, tone: &str, autonomy: &str) -> Result<Gaian, &'static str> {
    if id.trim().is_empty() {
        return Err("an identity is required");
    }
    if autonomy == "unbounded" {
        return Err("unbounded is refused");
    }
    Ok(Gaian { id: id.to_string(), tone: tone.to_string(), autonomy: autonomy.to_string() })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unbounded_is_refused_and_the_provider_can_miss() {
        assert_eq!(new("ada", "plain", "unbounded"), Err("unbounded is refused"));
        let gaian = new("ada", "plain", "observe").unwrap();
        assert_eq!(gaian.id, "ada");
        assert_eq!(MissingLocal.complete("hello"), Err("local model did not answer"));
    }
}
