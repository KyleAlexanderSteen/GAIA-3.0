# Universal AI Governance & Threat Control Matrix

**Issue:** #1643  
**Status:** specification/conformance draft  
**Scope:** GAIA 3.0 AI governance, agent security, adversarial AI, supply-chain integrity, human oversight, epistemic governance, monitoring, audit, incident response, and recovery.

## 1. Purpose

This matrix establishes a GAIA-wide governance and threat-control vocabulary for AI systems, agents, models, tools, skills, MCP servers, services, and autonomous workflows.

It is deliberately **specification/conformance work first**. Existing GAIA capabilities are identified before new implementation is proposed. Historical lineage is treated as evidence and conformance material rather than discarded.

The matrix does not authorize autonomous defensive or offensive behavior.

## 2. Trust posture

GAIA uses the following default characterization sequence:

```
Unknown
  ↓
Untrusted
  ↓
Characterize
  ↓
Evaluate
  ↓
Authorize or Restrict
```

Unknown does not mean malicious. It means that authority has not yet been established.

Labels such as "shadow AI" or "dark-web AI" describe deployment or threat context, not an intrinsic class of intelligence. Controls therefore operate on observable identity, provenance, capability, authority, behavior, scope, effects, evidence, and recovery properties.

## 3. Universal semantic mapping

Every control is mapped to the Universal GAIA semantic kernel:

```
ENTITY
IDENTITY
STATE
RELATION
CAPABILITY
AUTHORITY
AUTHORIZATION
ACTION
EFFECT
EVIDENCE
VERIFICATION
TRANSITION / RECOVERY
```

The following distinctions are normative:

- Capability ≠ Authority
- Authority ≠ Authorization
- Authorization ≠ Consent
- Intent ≠ Action
- Action ≠ Effect
- Expected Effect ≠ Observed Effect
- Observation ≠ Evidence
- Evidence ≠ Verification
- Verification ≠ Authority
- Reversibility ≠ Recovery

## 4. Existing GAIA foundation

The audit found substantial existing security/governance material.

| Existing surface | Evidence | Matrix role |
|---|---|---|
| Universal GAIA Model | `docs/architecture/UNIVERSAL_GAIA_MODEL_v0.1.md` | Canonical semantic boundary |
| SOS ABI | `gaia-spec/sos/abi.md` | Intent, authority intersection, deny-by-default |
| Capability audit | `docs/architecture/capability-audit/README.md` | Claim → implementation → test → evidence |
| Untrusted-content threat model | `docs/security/UNTRUSTED_CONTENT_THREAT_MODEL.md` | Prompt/tool/content trust boundary |
| Rogue-agent threat model | `docs/security/ROGUE_AGENT_THREAT_MODEL.md` | Objective drift, safeguard reduction, credential pivot, C2, persistence |
| Agent tool control plane | `docs/security/AGENT_TOOL_CONTROL_PLANE.md` | Deterministic gateway and approval model |
| Lifecycle/promotion controls | `tools/agent-skills/*` and lifecycle workflows | Evidence and promotion governance |
| Historical GAIA lineage | `docs/architecture/GAIA_ARCHITECTURAL_LINEAGE_AND_CONFORMANCE.md` | Existing implementation/specification evidence |
| Vendor due diligence | `docs/governance/vendor-due-diligence.md` | Supplier governance precedent |

**Finding:** #1643 does not justify a new universal security ontology. It primarily requires normalization, crosswalk, conformance testing, and targeted gap closure.

## 5. Control matrix

### GOV-01 — Identity and provenance

**Threat:** anonymous, impersonated, stale, or unverifiable AI actors.

**Control:** Every governed AI actor, tool, skill, server, model, and service must have an attributable identity/provenance record appropriate to its risk and scope.

**Universal mapping:** ENTITY, IDENTITY, RELATION, EVIDENCE.

**Existing evidence:** GAIA identity modules; signed intent subject identity; historical provenance/audit systems.

**Verification:** identity binding, signature/issuer validation where applicable, provenance continuity, revocation tests.

**Recovery:** revoke identity/session/credential and preserve forensic evidence.

### GOV-02 — Capability declaration and least privilege

**Threat:** capability expansion beyond declared task requirements.

**Control:** Capabilities must be explicit, scoped, time-bounded where applicable, and denied by default when not authorized.

**Universal mapping:** CAPABILITY, AUTHORITY, AUTHORIZATION.

**Existing evidence:** permissions, capability manifests, SOS ABI authority intersection, agent control plane.

**Verification:** out-of-scope request fixtures and negative authorization tests.

**Recovery:** deny, revoke expanded scope, record evidence.

### GOV-03 — Authority and authorization separation

**Threat:** treating capability, role, policy, consent, or model output as automatic authority.

**Control:** Authority is established independently from capability; authorization is an evaluated decision for a specific normalized action.

**Universal mapping:** CAPABILITY, AUTHORITY, AUTHORIZATION, ACTION.

**Existing evidence:** SOS ABI effective-authority intersection; deterministic gateway decision states.

**Verification:** tests proving each layer can be denied independently.

### GOV-04 — Consent and human accountability

**Threat:** AI action affecting a person without appropriate consent or accountable authorization.

**Control:** Consent remains distinct from authority and must be represented explicitly where required. Human approval must bind to the exact action, target, scope, and expiry when required by policy.

**Universal mapping:** AUTHORIZATION, RELATION, EVIDENCE.

**Existing evidence:** historical consent guards/ledger; HumanApprovalReceipt contract.

**Verification:** revoked consent, changed-target approval, replayed approval, and expiry tests.

### GOV-05 — Agent behavior and tool-use controls

**Threat:** an authorized agent deviates from its approved objective or tool scope.

**Control:** Continuously re-evaluate objective, constraint level, credential scope, egress destination, write target, and task lifetime.

**Universal mapping:** STATE, CAPABILITY, AUTHORIZATION, ACTION, EFFECT, VERIFICATION.

**Existing evidence:** rogue-agent threat model.

**Verification:** RC-1 through RC-5 fixture corpus.

**Recovery:** circuit breaker, halt, revocation, human review, restoration.

### GOV-06 — Prompt and instruction security

**Threat:** direct or indirect prompt injection changes policy, scope, tool selection, or authorization.

**Control:** External text and model proposals are data, not authority. Policy and authorization state must remain outside model-controlled context.

**Universal mapping:** RELATION, CAPABILITY, AUTHORIZATION, ACTION, EVIDENCE.

**Existing evidence:** `UNTRUSTED_CONTENT_THREAT_MODEL.md`.

**Verification:** README, issue, PR, tool-output, log, and retrieved-content injection fixtures.

### GOV-07 — Data security and privacy

**Threat:** unauthorized collection, disclosure, credential access, cross-agent leakage, or data exfiltration.

**Control:** Data access is scoped to identity, capability, authorization, purpose, target, and egress policy. Secrets must not be inherited implicitly.

**Universal mapping:** ENTITY, IDENTITY, CAPABILITY, AUTHORIZATION, ACTION, EFFECT.

**Existing evidence:** secret boundary, egress controls, protected-path policy, historical consent controls.

**Verification:** secret-access denial, cross-scope access, egress, and redaction tests.

### GOV-08 — Tool, skill, and MCP security

**Threat:** compromised tools, malicious skills, poisoned metadata, unsafe tool outputs, or unauthorized server selection.

**Control:** Tool identity/version, capability, parameters, destination, and outputs must be independently validated. Tool metadata cannot grant authority.

**Universal mapping:** ENTITY, IDENTITY, CAPABILITY, AUTHORIZATION, ACTION, EVIDENCE.

**Existing evidence:** agent tool control plane and untrusted-content model.

**Verification:** metadata poisoning, output injection, server substitution, scope expansion, and version-integrity tests.

### GOV-09 — Code-execution isolation

**Threat:** tool or agent execution escapes its sandbox or gains host privileges.

**Control:** Execution environments must use explicit resource, filesystem, process, credential, and network boundaries appropriate to risk.

**Universal mapping:** CAPABILITY, AUTHORITY, ACTION, EFFECT, TRANSITION/RECOVERY.

**Existing evidence:** sandbox/egress boundary design in agent control plane.

**Verification:** path, process, credential, host, network, and persistence escape fixtures.

### GOV-10 — Supply-chain integrity

**Threat:** malicious or mutable model, package, dependency, skill, tool, CI, or configuration supply chain.

**Control:** Pin and verify dependencies and artifacts according to risk; preserve provenance and detect unexpected changes.

**Universal mapping:** ENTITY, IDENTITY, RELATION, EVIDENCE, VERIFICATION.

**Existing evidence:** vendor due diligence; provenance and lifecycle/audit foundations.

**Verification:** mutation, provenance mismatch, unsigned/unverified artifact, and dependency-policy tests.

### GOV-11 — Shadow AI discovery and characterization

**Threat:** unregistered AI systems or agents operate without governance visibility.

**Control:** Discover and inventory AI systems, tools, models, agents, and autonomous workflows where feasible; classify them by observable identity, capability, data access, authority, deployment context, and risk.

**Universal mapping:** ENTITY, IDENTITY, STATE, CAPABILITY, RELATION, EVIDENCE.

**Existing evidence:** historical AI System Inventory work and current capability audit.

**Important boundary:** discovery does not itself authorize blocking, surveillance, or intervention. Any response must have its own authority.

### GOV-12 — Adversarial AI threat modeling

**Threat:** adversarial behavior against or through AI systems.

**Control:** Maintain a threat taxonomy covering prompt injection, poisoning, credential access, persistence, privilege escalation, lateral movement, exfiltration, C2, supply-chain attacks, and harmful autonomous behavior.

**Universal mapping:** STATE, RELATION, ACTION, EFFECT, EVIDENCE, VERIFICATION.

**Existing evidence:** rogue-agent and untrusted-content threat models.

**External crosswalk:** MITRE ATLAS provides a living taxonomy of AI adversary tactics/techniques and includes agentic-AI techniques. It is a reference source, not GAIA authority.

### GOV-13 — Agent-to-agent trust and delegation

**Threat:** privilege laundering, impersonation, recursive delegation, or inherited authority through agent output.

**Control:** Agent outputs remain untrusted unless independently validated. Delegation must explicitly identify delegator, delegatee, scope, target, duration, constraints, and authorization.

**Universal mapping:** ENTITY, IDENTITY, RELATION, CAPABILITY, AUTHORITY, AUTHORIZATION.

**Existing evidence:** cross-agent laundering controls in untrusted-content model; signed identity/manifest concepts.

**Verification:** delegation without authority, recursive delegation, scope widening, and impersonation tests.

### GOV-14 — Human oversight and escalation

**Threat:** high-impact decisions execute without appropriate human review or escalation.

**Control:** Risk policy determines when human approval, two-person review, or prohibition is required. Approval must be exact, attributable, bounded, and auditable.

**Universal mapping:** AUTHORIZATION, EVIDENCE, VERIFICATION, TRANSITION/RECOVERY.

**Existing evidence:** HumanApprovalReceipt and historical Human Oversight Log work.

### GOV-15 — Safety, ethics, and non-harm

**Threat:** technically authorized behavior produces prohibited or unjustified harm.

**Control:** Policy must evaluate safety, impact, consent, rights, affected entities, and applicable constraints before high-impact actions. GAIA's foundational principle remains: **do no harm, and value all life, whether alive or not.**

**Universal mapping:** AUTHORITY, AUTHORIZATION, ACTION, EFFECT, EVIDENCE, VERIFICATION.

**Boundary:** ethical principles do not replace concrete safety engineering, law, or domain-specific controls.

### GOV-16 — Epistemic governance

**Threat:** unsupported claims or model uncertainty are treated as verified facts or authority.

**Control:** Separate observation, claim, inference, evidence, verification, and authority. Preserve uncertainty and provenance.

**Universal mapping:** EVIDENCE, VERIFICATION, RELATION, AUTHORIZATION.

**Existing evidence:** epistemic state layer; capability audit; historical epistemic governance.

**Verification:** tests preventing specification, model output, or narrative confidence from becoming authorization evidence.

### GOV-17 — Monitoring and anomaly detection

**Threat:** unauthorized state transitions or behavior remain undetected.

**Control:** Monitor governed actions and relevant state transitions for authorization mismatch, objective drift, unexpected execution paths, resource anomalies, and repeated failures.

**Universal mapping:** STATE, ACTION, EFFECT, EVIDENCE, VERIFICATION.

**Existing evidence:** #1604 self-detection/stability work; rogue-agent telemetry requirements.

**Boundary:** anomaly detection is evidence for review, not automatic proof of malicious intent.

### GOV-18 — Audit, provenance, and forensics

**Threat:** inability to reconstruct who/what authorized, attempted, executed, or affected an action.

**Control:** Consequential decisions and actions produce privacy-minimized, attributable, correlated, tamper-evident evidence without granting new authority through receipts.

**Universal mapping:** IDENTITY, RELATION, ACTION, EFFECT, EVIDENCE, VERIFICATION.

**Existing evidence:** audit/event fabric; ActionReceipt; provenance models.

**Verification:** correlation, integrity, completeness, privacy, and replay/reconstruction tests.

### GOV-19 — Incident response and containment

**Threat:** unsafe behavior persists after detection or denial.

**Control:** Define deterministic halt, deny, revoke, quarantine, escalation, and evidence-preservation paths for material violations.

**Universal mapping:** AUTHORIZATION, ACTION, EFFECT, TRANSITION/RECOVERY.

**Existing evidence:** rogue-agent circuit-breaker posture; historical containment manager.

**Verification:** injected rogue conditions and containment tests.

### GOV-20 — Resilience, recovery, and restoration

**Threat:** recovery reintroduces compromised state or loses accountability evidence.

**Control:** Recovery must be an explicit state transition with authorization, provenance, validation, and restoration criteria. Reversibility is not assumed to equal recovery.

**Universal mapping:** STATE, EFFECT, EVIDENCE, VERIFICATION, TRANSITION/RECOVERY.

**Existing evidence:** historical containment/restoration; current lifecycle/promotion controls.

**Verification:** compromised-state restoration, rollback integrity, evidence preservation, and recovery authorization tests.

### GOV-21 — AI lifecycle governance

**Threat:** AI systems change models, prompts, tools, policies, dependencies, or deployment state without renewed governance.

**Control:** Governance applies across acquisition, development, testing, deployment, operation, change, suspension, retirement, and archival.

**Universal mapping:** STATE, RELATION, AUTHORIZATION, EVIDENCE, VERIFICATION, TRANSITION/RECOVERY.

**Existing evidence:** lifecycle and promotion-readiness machinery; capability audit.

### GOV-22 — Governance integrity / who governs the governor?

**Threat:** the governed agent can modify its own policy, audit, identity, authorization, or oversight boundary.

**Control:** Governance mechanisms must themselves be attributable, versioned, protected, independently evaluated where appropriate, and subject to external revocation. The audited agent cannot unilaterally disable the controls auditing it.

**Universal mapping:** AUTHORITY, AUTHORIZATION, EVIDENCE, VERIFICATION, TRANSITION/RECOVERY.

**Existing evidence:** protected policy paths, deterministic gateway, human stop/revocation, lifecycle governance.

**Verification:** policy-tamper, audit-disable, self-authorization, and governance-recursion tests.

## 6. Threat-to-control crosswalk

| Threat class | Primary controls | Required evidence |
|---|---|---|
| Unknown/unregistered AI | GOV-01, 02, 11, 21 | Identity/inventory record and authorization disposition |
| Prompt injection | GOV-06, 08, 16 | Negative fixture and independent policy decision |
| Malicious tool/skill/MCP | GOV-01, 08, 10 | Verified identity/version + bounded manifest |
| Credential pivoting | GOV-02, 05, 07, 13 | Credential-scope denial and audit record |
| C2 / egress abuse | GOV-05, 07, 09, 19 | Egress-denial evidence and containment |
| Persistence | GOV-05, 09, 19, 20 | Protected-path/process denial and recovery record |
| Supply-chain poisoning | GOV-01, 10, 21 | Provenance/integrity verification |
| Cross-agent privilege laundering | GOV-01, 02, 13 | Delegation and scope-boundary tests |
| Governance tampering | GOV-18, 21, 22 | Protected-policy and audit-integrity evidence |
| Epistemic overclaim | GOV-15, 16, 17 | Evidence/verification separation |
| Unauthorized high-impact action | GOV-03, 04, 14, 15 | Exact approval/deny evidence |
| Recovery into compromised state | GOV-18, 19, 20 | Restoration verification and preserved forensic record |

## 7. External framework crosswalk

### NIST AI RMF / GenAI Profile

NIST AI RMF is a voluntary risk-management framework intended to help manage AI risks across design, development, deployment, use, and evaluation. Its GenAI Profile adds risks and suggested actions specific to generative AI. NIST also launched an AI Agent Standards Initiative in 2026 focused in part on secure agent standards, interoperability, identity, and security research.

GAIA correspondence:

- **Govern:** GOV-01, 03, 04, 15, 21, 22
- **Map:** GOV-10, 11, 12, 16, 17
- **Measure:** GOV-05, 16, 17, 18
- **Manage:** GOV-02, 06–10, 13–15, 19–20

GAIA adds a stronger semantic authorization boundary by explicitly separating capability, authority, authorization, consent, action, effect, evidence, and verification.

### OWASP Top 10 for Agentic Applications

OWASP's 2026 Agentic Applications framework addresses security risks specific to autonomous/agentic systems. GAIA crosswalks its themes into GOV-05–09, 12–13, 17–20, rather than copying the list as a GAIA ontology.

### OWASP Agentic Skills Top 10

OWASP's Agentic Skills project focuses on the execution/behavior layer of agent skills between model reasoning and tool invocation. GAIA maps that layer primarily to capability declaration, skill/tool identity, authorization, action/effect evidence, supply-chain integrity, and lifecycle controls.

### OWASP MCP Top 10

MCP-specific risks are represented through GOV-08, GOV-09, GOV-10, GOV-13, and GOV-18. MCP metadata and tool output remain untrusted until independently validated.

### MITRE ATLAS

MITRE ATLAS is a living, threat-informed knowledge base of tactics and techniques involving AI systems. Its current matrix includes agentic-AI behaviors such as tool invocation, context/tool poisoning, credential harvesting, configuration modification, persistence, C2, and exfiltration.

GAIA uses ATLAS for threat enumeration and red-team/test coverage. ATLAS classifications do not themselves grant authority or determine GAIA policy.

## 8. Evidence status model

Every control claim must use one of:

- **IMPLEMENTED** — executable behavior exists with appropriate verification evidence.
- **SPECIFIED** — architecture/contract exists but implementation evidence is insufficient.
- **PARTIAL** — some behavior exists but important controls remain incomplete.
- **UNVERIFIED** — implementation may exist, but required evidence has not been established.
- **GAP** — after historical/current audit, an adequate control is not found.
- **NOT_APPLICABLE** — control is explicitly outside the governed scope with rationale.

A documentation claim alone is never sufficient for IMPLEMENTED.

## 9. Required control record

Each machine-readable record contains:

```text
Control ID
Domain
Universal primitive(s)
Threat
Existing GAIA evidence
Status
Authority boundary
Evidence requirement
Verification
Recovery
Provenance
Notes
```

## 10. Non-negotiable invariants

1. Default deny for unauthorized capability.
2. Unknown identity receives no ambient authority.
3. AI cannot unilaterally grant itself authority.
4. AI cannot silently modify its own governing policy.
5. Audit controls cannot be disabled by the agent being audited.
6. Governance changes are versioned and attributable.
7. Critical authorization decisions produce evidence.
8. Revocation is enforceable.
9. High-impact actions require appropriate authorization and escalation.
10. External content is untrusted until evaluated.
11. Security claims require executable or independently reviewable evidence.
12. Symbolic, philosophical, experimental, or hypothetical claims do not silently become operational facts.
13. Safety controls have a recovery path.
14. No single AI component is an unreviewable root of trust.
15. Governance must preserve human agency and must not become covert manipulation.

## 11. Genuine-gap determination

The initial audit does **not** establish a new universal semantic primitive.

Likely follow-up work should focus on conformance and integration questions:

- common control-record schema validation;
- cross-document control-ID consistency;
- executable fixtures for the existing rogue-agent and untrusted-content contracts;
- agent-to-agent delegation conformance;
- shadow-AI inventory/discovery semantics;
- governance-change protection and independent verification;
- lifecycle integration of AI governance evidence;
- NIST/OWASP/ATLAS machine-readable crosswalk maintenance.

A follow-up implementation issue is justified only when the missing behavior is demonstrated to be a genuine gap after existing and historical evidence is checked.

## 12. Safety boundary

This matrix does not authorize:

- autonomous cyber offense;
- autonomous retaliation;
- uncontrolled surveillance;
- privilege escalation;
- persistence;
- credential harvesting;
- arbitrary external execution;
- disabling safety controls.

Threat descriptions are for governance, testing, detection, containment, and defensive verification.

## 13. Research references

- NIST AI RMF: https://www.nist.gov/itl/ai-risk-management-framework
- NIST AI RMF GenAI Profile: https://www.nist.gov/publications/artificial-intelligence-risk-management-framework-generative-artificial-intelligence
- NIST AI Agent Standards Initiative: https://www.nist.gov/news-events/news/2026/02/announcing-ai-agent-standards-initiative-interoperable-and-secure
- OWASP Top 10 for Agentic Applications 2026: https://genai.owasp.org/resource/owasp-top-10-for-agentic-applications-for-2026/
- OWASP Agentic Skills Top 10: https://owasp.org/projects/agentic-skills-top-10
- OWASP MCP Top 10: https://owasp.org/projects/mcp-top-10
- MITRE ATLAS: https://atlas.mitre.org/

## 14. Disposition

This document is a **GAIA 3.0 specification/conformance artifact**.

It normalizes existing GAIA security/governance work and establishes a machine-readable control registry boundary. It does not claim that every listed control is already implemented.

Follow-up issues must reference #1643 and identify the exact evidence-backed gap they address.
