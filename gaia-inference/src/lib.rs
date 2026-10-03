//! Local inference. A remote provider is not implemented.

use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Completion {
    pub text: String,
}

pub fn complete_local(prompt: &str) -> Result<Completion, String> {
    if prompt.trim().is_empty() {
        return Err("empty prompt".into());
    }

    let body = format!(
        "{{\"model\":\"llama3.2\",\"prompt\":{},\"stream\":false}}",
        serde_json::to_string(prompt).map_err(|err| err.to_string())?
    );
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(2))
        .build();
    let response = agent
        .post("http://127.0.0.1:11434/api/generate")
        .set("content-type", "application/json")
        .send_string(&body)
        .map_err(|_| "local model did not answer".to_string())?;
    let text = response
        .into_string()
        .map_err(|err| err.to_string())?;
    Ok(Completion { text })
}

pub fn remote(_name: &str) -> Result<Completion, String> {
    Err("remote provider is not implemented".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_prompt_and_a_remote_provider_fail() {
        assert_eq!(complete_local("").unwrap_err(), "empty prompt");
        assert_eq!(
            remote("openai").unwrap_err(),
            "remote provider is not implemented"
        );
    }
}
