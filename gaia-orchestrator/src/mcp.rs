use crate::intent::IntentGraph;
use crate::trust::{verify_tagged_signature, IntentSigner};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct McpTool {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct McpResource {
    pub uri: String,
    pub name: String,
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aip_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trust: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AipManifest {
    pub name: String,
    pub version: String,
    pub tools: Vec<McpTool>,
    pub resources: Vec<McpResource>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct McpMessage {
    pub jsonrpc: String,
    pub method: String,
    pub params: String,
    pub signature: Option<String>,
}

impl McpMessage {
    pub fn unsigned(method: &str, params: &str) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            method: method.into(),
            params: params.into(),
            signature: None,
        }
    }

    pub fn marked_signed(method: &str, params: &str, token: &str) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            method: method.into(),
            params: params.into(),
            signature: Some(token.into()),
        }
    }

    pub fn signed_by(signer: &IntentSigner, method: &str, params: &str) -> Self {
        let payload = format!("{method}\n{params}");
        Self {
            jsonrpc: "2.0".into(),
            method: method.into(),
            params: params.into(),
            signature: Some(signer.sign_bytes(payload.as_bytes())),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    pub params: String,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: u64,
    pub result: Option<String>,
    pub error: Option<String>,
}

impl JsonRpcRequest {
    pub fn signed(signer: &IntentSigner, id: u64, method: &str, params: &str) -> Self {
        let payload = format!("{method}\n{params}");
        Self {
            jsonrpc: "2.0".into(),
            id,
            method: method.into(),
            params: params.into(),
            signature: Some(signer.sign_bytes(payload.as_bytes())),
        }
    }

    pub fn unsigned(id: u64, method: &str, params: &str) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id,
            method: method.into(),
            params: params.into(),
            signature: None,
        }
    }

    fn as_message(&self) -> McpMessage {
        McpMessage {
            jsonrpc: self.jsonrpc.clone(),
            method: self.method.clone(),
            params: self.params.clone(),
            signature: self.signature.clone(),
        }
    }
}

fn public_hex_from_tag(tagged: &str) -> Option<&str> {
    let mut parts = tagged.splitn(3, ':');
    let alg = parts.next()?;
    let hex = parts.next()?;
    let _sig = parts.next()?;
    if alg == "ed25519" && !hex.is_empty() {
        Some(hex)
    } else {
        None
    }
}

#[derive(Debug, Clone)]
pub struct McpRegistry {
    pub agents: Vec<AipManifest>,
    pub reject_unsigned: bool,
    pub trusted_keys: Vec<String>,
    pub revoked_keys: Vec<String>,
    pub handler_invocations: u32,
    pub audit: Vec<String>,
}

impl Default for McpRegistry {
    fn default() -> Self {
        Self {
            agents: Vec::new(),
            reject_unsigned: true,
            trusted_keys: Vec::new(),
            revoked_keys: Vec::new(),
            handler_invocations: 0,
            audit: Vec::new(),
        }
    }
}

impl McpRegistry {
    pub fn local() -> Self {
        Self {
            reject_unsigned: true,
            trusted_keys: Vec::new(),
            revoked_keys: Vec::new(),
            handler_invocations: 0,
            audit: Vec::new(),
            agents: vec![AipManifest {
                name: "gaia-local-researcher".into(),
                version: "0.1.0".into(),
                tools: vec![McpTool {
                    name: "research.summarize".into(),
                    description: "Local research+summarize stub".into(),
                }],
                resources: vec![McpResource {
                    uri: "gaia://agent/agent.researcher".into(),
                    name: "agent.researcher".into(),
                    kind: "agent".into(),
                    aip_ref: Some("gaia-spec/examples/cognitive-agent.json".into()),
                    capabilities: Some(vec!["research.summarize".into()]),
                    trust: Some("local-stub".into()),
                    endpoint: Some("in-process".into()),
                }],
            }],
        }
    }

    pub fn trust(mut self, signer: &IntentSigner) -> Self {
        self.trusted_keys.push(signer.principal().public_hex());
        self
    }

    pub fn revoke(mut self, signer: &IntentSigner) -> Self {
        self.revoked_keys.push(signer.principal().public_hex());
        self
    }

    pub fn live_wire(&self) -> bool {
        false
    }

    pub fn listen_tcp(&self, _bind: &str) -> Result<(), String> {
        Err("MCP is in-process only; no TCP listen".into())
    }

    pub fn serve_once<R: std::io::BufRead, W: std::io::Write>(
        &mut self,
        reader: &mut R,
        writer: &mut W,
    ) -> Result<(), String> {
        crate::mcp_stdio::serve_once(self, reader, writer)
    }

    pub fn list(&self) -> &[AipManifest] {
        &self.agents
    }

    pub fn tools(&self) -> Vec<&McpTool> {
        self.agents.iter().flat_map(|a| a.tools.iter()).collect()
    }

    pub fn resources(&self) -> Vec<&McpResource> {
        self.agents
            .iter()
            .flat_map(|a| a.resources.iter())
            .collect()
    }

    pub fn admit(&mut self, msg: &McpMessage) -> Result<(), String> {
        if self.reject_unsigned && msg.signature.is_none() {
            self.audit.push("GAIA_UNAUTHENTICATED".into());
            return Err("GAIA_UNAUTHENTICATED: unsigned MCP traffic rejected".into());
        }
        if msg.jsonrpc != "2.0" {
            return Err("jsonrpc 2.0 required".into());
        }
        if let Some(signature) = &msg.signature {
            let payload = format!("{}\n{}", msg.method, msg.params);
            if verify_tagged_signature(payload.as_bytes(), signature).is_err() {
                self.audit.push("GAIA_SIGNATURE_INVALID".into());
                return Err("GAIA_SIGNATURE_INVALID: forged or tampered MCP rejected".into());
            }
            if let Some(key) = public_hex_from_tag(signature) {
                if self.revoked_keys.iter().any(|k| k == key) {
                    self.audit.push("GAIA_IDENTITY_UNTRUSTED".into());
                    return Err("GAIA_IDENTITY_UNTRUSTED: revoked principal".into());
                }
                if !self.trusted_keys.is_empty() && !self.trusted_keys.iter().any(|k| k == key) {
                    self.audit.push("GAIA_IDENTITY_UNTRUSTED".into());
                    return Err("GAIA_IDENTITY_UNTRUSTED: unknown principal".into());
                }
            }
        }
        Ok(())
    }

    fn policy_allows(&self, params: &str) -> Result<(), String> {
        if params.contains("\"network\"") || params.contains("jurisdiction\":\"remote\"") {
            return Err("GAIA_POLICY_DENIED: capability or jurisdiction not allowed".into());
        }
        Ok(())
    }

    fn dispatch_tool(&mut self, name: &str, intent_id: &str) -> Result<String, String> {
        if !self.tools().iter().any(|t| t.name == name) {
            return Err("GAIA_RESOURCE_NOT_FOUND: tool not in registry".into());
        }
        self.handler_invocations += 1;
        let audit_id = format!("mcp-audit-{}", self.handler_invocations);
        self.audit.push(format!("ADMIT {name} {audit_id}"));
        Ok(format!(
            "mcp-not-executed:{name};intent={intent_id};resource=gaia://agent/agent.researcher;provenance=local-stub;quality=fixture;freshness=static;audit={audit_id}"
        ))
    }

    pub fn invoke(&mut self, msg: &McpMessage) -> Result<String, String> {
        self.admit(msg)?;
        self.policy_allows(&msg.params)?;
        let tool = if msg.method == "tools/call" {
            msg.params.clone()
        } else {
            msg.method.clone()
        };
        self.dispatch_tool(&tool, "unbound")
    }

    pub fn invoke_from_intent(
        &mut self,
        graph: &IntentGraph,
        signer: &IntentSigner,
    ) -> Result<String, String> {
        let method = if graph.goal.to_ascii_lowercase().contains("research") {
            "research.summarize"
        } else {
            return Err("no MCP tool mapped for intent".into());
        };
        let msg = McpMessage::signed_by(signer, method, &graph.goal);
        self.admit(&msg)?;
        self.dispatch_tool(method, &graph.id.to_string())
    }

    pub fn handle(&mut self, req: JsonRpcRequest) -> JsonRpcResponse {
        let fail = |error: String| JsonRpcResponse {
            jsonrpc: "2.0".into(),
            id: req.id,
            result: None,
            error: Some(error),
        };
        if let Err(error) = self.admit(&req.as_message()) {
            return fail(error);
        }
        if req.method == "tools/call" {
            if let Err(error) = self.policy_allows(&req.params) {
                self.audit.push("GAIA_POLICY_DENIED".into());
                return fail(error);
            }
        }
        let result = match req.method.as_str() {
            "initialize" => Ok("protocol=2026-07-28 transport=in-process".into()),
            "tools/list" => serde_json::to_string(&self.tools()).map_err(|e| e.to_string()),
            "resources/list" => serde_json::to_string(&self.resources()).map_err(|e| e.to_string()),
            "tools/call" => self.dispatch_tool(&req.params, "rpc"),
            _ => Err(format!("unknown method {}", req.method)),
        };
        match result {
            Ok(value) => JsonRpcResponse {
                jsonrpc: "2.0".into(),
                id: req.id,
                result: Some(value),
                error: None,
            },
            Err(error) => fail(error),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DiscoveryStub;

impl DiscoveryStub {
    pub fn advertise(&self, _manifest: &AipManifest) -> Result<(), String> {
        Ok(())
    }

    pub fn browse_local<'a>(&self, registry: &'a McpRegistry) -> &'a [AipManifest] {
        registry.list()
    }

    pub fn browse_mdns(&self) -> Result<(), String> {
        Err("mDNS/DHT not on the wire in v0".into())
    }
}
