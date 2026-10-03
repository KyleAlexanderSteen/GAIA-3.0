# RFC 0003: Read-Only GitHub Source Access Node

- Status: Proposed
- Issue: #305
- Authors: GAIA 2.0 contributors
- Created: 2026-09-13
- Decision type: Architecture and security boundary

## Summary

GAIA SHALL provide a `gaia-github-source` provider adapter for retrieving bounded, attributable repository knowledge. The adapter is read-only. It is a source capability for documentation, specifications, source code, commits, pull requests, issues, and immutable snapshots; it is not a GitHub automation agent.

The initial installation SHALL be limited to `KyleAlexanderSteen/GAIA-2.0`.

## Motivation

GAIA needs durable access to its own evolving source and research corpus so that GAIANs and specialist agents can reason over implementation and documentation with revision-level provenance. Raw, unbounded repository access is incompatible with the project's local-first, sovereign, explicitly approved orchestration model.

A narrowly defined source node permits reproducible research and implementation support without granting authority to create branches, modify code, alter workflows, access secrets, or administer repositories.

## Decision

The source path is:

```text
GAIA CLI / Interface
  -> intent and trust policy
  -> orchestrator source-capability registry
  -> gaia-github-source
  -> GitHub App / GitHub API
```

`gaia-github-source` SHALL perform only the following logical operations:

```text
github.source.repository.get
github.source.tree.list
github.source.file.read
github.source.commit.get
github.source.pull_request.read
github.source.issue.read
github.source.search.code
github.source.search.docs
github.source.snapshot.create
```

The node SHALL have no operation that changes state at GitHub.

## Capability policy

### Repository and path bounds

The server-side policy SHALL enforce a repository allowlist. Initial value:

```text
KyleAlexanderSteen/GAIA-2.0
```

Initial documentation paths SHALL be:

```text
Documents/**
Documents-2/**
gaia-spec/**
rfcs/**
README.md
CONTRIBUTING.md
SECURITY.md
GOVERNANCE.md
```

Code paths MAY be added only by a policy change reviewed through the normal GAIA change process. A caller supplied repository, ref, or path is never an authorization grant.

### Provider credentials

The implementation SHALL use a dedicated GitHub App installed only on allowlisted repositories. It SHALL request only the minimum read permissions needed for the enabled operations:

- Repository metadata: read
- Contents: read
- Pull requests: read
- Issues: read
- Commit status/check information: read when required

The adapter SHALL NOT request or use contents write, pull request write, issue write, workflow write, actions write, administration, deployments, secrets, webhooks write, or a broad personal access token. Installation tokens SHALL be obtained through a dedicated credential boundary, be short lived, and never be emitted in responses, logs, audit events, cache records, or prompts.

### Content bounds and refusals

The node SHALL apply maximum response size, maximum file size, content-type allowlists, request rate limits, and pagination limits. It SHALL deny or redact content identified as likely credentials, private keys, tokens, environment files, or repository-secret material. Denials SHALL be explicit and audit-recorded without recording the sensitive content.

## Provenance envelope

Each successful source result SHALL include enough information to reproduce and evaluate it:

```json
{
  "source": "github",
  "repository": "KyleAlexanderSteen/GAIA-2.0",
  "ref": "main",
  "path": "gaia-spec/intent-graph.md",
  "commit_sha": "...",
  "blob_sha": "...",
  "retrieved_at": "RFC3339 timestamp",
  "content_type": "text/markdown",
  "content_hash": "sha256:...",
  "source_url": "https://github.com/...",
  "content": "bounded content or content reference"
}
```

A symbolic branch ref SHALL be resolved to a commit SHA before cache storage or agent use. The implementation SHALL state when a result was truncated, omitted, redacted, unavailable, or stale.

## Cache and audit

The local cache SHALL be content-addressed by immutable repository identity, commit SHA, path, blob SHA where present, and content hash. A cache response SHALL never be returned for a different resolved revision.

The node SHALL append a local audit event for each request outcome. The event SHALL capture the requesting capability or signed intent identifier where available, allowed/denied outcome, repository/ref/path selector, resolved immutable identifiers, policy decision, retrieval time, and response-content hash. It SHALL not store token values or denied sensitive content. Audit records SHALL participate in GAIA's existing tamper-evident/hash-chained audit model where supported.

## Orchestration integration

The node is a future governed capability consumer of the local, signed intent-to-MCP routing introduced by PR #306. This RFC does not modify that path and does not authorize remote transport, TCP listening, mDNS/DHT discovery, or autonomous GitHub mutation.

Source retrieval may be invoked only after the relevant trust and policy checks. Reading public documentation is still a controlled capability because scope, data minimization, reproducibility, and auditability are architectural requirements.

## Non-goals

This RFC does not define:

- A GitHub write capability.
- Autonomous commits, branch creation, pull requests, comments, labels, or reviews.
- Repository administration, workflow modification, deployment control, webhook administration, or secret access.
- A general cross-repository crawler or organization-wide index.
- Streamable HTTP, remote MCP, or network discovery.
- Storage of GitHub credentials in GAIA memory or prompt context.

A future `gaia-github-change-broker` SHALL be a separate design and implementation. It SHALL require a signed intent, exact resolved target, complete proposed mutation, explicit human approval, narrowly scoped short-lived credentials, and post-write audit.

## Rollout

1. Define a versioned source-result schema and policy configuration in `gaia-spec`.
2. Add unit tests for repository/path allowlists, ref resolution, provenance, truncation, cache isolation, and audit events.
3. Implement documentation-only reads using a test double before live GitHub credentials.
4. Add the dedicated GitHub App and a credential boundary after a security review.
5. Expand to explicitly approved code paths only after documentation reads and audits are verified.

## Acceptance criteria

- [ ] Calls outside the repository allowlist are denied.
- [ ] Calls outside the configured path allowlist are denied.
- [ ] Every successful result contains repository, resolved revision, path, timestamp, source URL, and content hash.
- [ ] Cache keys isolate distinct refs and immutable revisions.
- [ ] Truncated, stale, redacted, and denied outcomes are distinguishable.
- [ ] Every request outcome has a local audit event with no credentials or secret contents.
- [ ] The implementation requests no GitHub write permission.
- [ ] Tests demonstrate that mutation operations cannot be reached through this node.
- [ ] A contract/schema review is completed before live GitHub App credentials are introduced.

## Consequences

GAIA gains a reproducible and governed way to read its own research and implementation corpus. The scope is intentionally narrower than a convenience integration: availability of source material does not confer authority to mutate the source of truth. The separation between a source node and a future change broker preserves human sovereignty and limits blast radius.
