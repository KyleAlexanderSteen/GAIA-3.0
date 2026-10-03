use gaia_orchestrator::{
    FakeGitHubSourceProvider, GitHubSourcePolicy, SourceAuditInput, SourceCacheKey, SourceOperation,
};

const REPO: &str = "KyleAlexanderSteen/GAIA-2.0";
const COMMIT: &str = "0000000000000000000000000000000000000000";
const HASH_A: &str = "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const HASH_B: &str = "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

#[test]
fn policy_allows_only_configured_repository_and_document_paths() {
    let policy = GitHubSourcePolicy::default();
    assert!(policy
        .authorize(REPO, "gaia-spec/intent-graph.md", SourceOperation::FileRead)
        .is_ok());
    assert!(policy
        .authorize(REPO, "Documents/GAIA.md", SourceOperation::FileRead)
        .is_ok());
    assert!(policy
        .authorize(
            "other/repo",
            "gaia-spec/intent-graph.md",
            SourceOperation::FileRead
        )
        .is_err());
    assert!(policy
        .authorize(
            REPO,
            "gaia-orchestrator/src/main.rs",
            SourceOperation::FileRead
        )
        .is_err());
}

#[test]
fn sensitive_paths_are_denied_even_when_they_look_documentary() {
    let policy = GitHubSourcePolicy::default();
    for path in [
        ".env",
        ".env.production",
        "Documents/key.pem",
        "rfcs/id_rsa",
        "gaia-spec/secrets/token",
    ] {
        assert!(
            policy
                .authorize(REPO, path, SourceOperation::FileRead)
                .is_err(),
            "{path}"
        );
    }
}

#[test]
fn only_read_operations_exist_in_the_contract() {
    let policy = GitHubSourcePolicy::default();
    assert!(policy.allows_operation(SourceOperation::RepositoryGet));
    assert!(policy.allows_operation(SourceOperation::SnapshotCreate));
}

#[test]
fn immutable_cache_key_requires_valid_ids_and_isolates_content() {
    let first =
        SourceCacheKey::from_immutable_revision(REPO, COMMIT, "gaia-spec/intent-graph.md", HASH_A)
            .unwrap();
    let second =
        SourceCacheKey::from_immutable_revision(REPO, COMMIT, "gaia-spec/intent-graph.md", HASH_B)
            .unwrap();
    assert_ne!(first, second);
    assert!(SourceCacheKey::from_immutable_revision(
        REPO,
        "main",
        "gaia-spec/intent-graph.md",
        HASH_A
    )
    .is_err());
}

#[test]
fn fake_provider_never_contacts_github_and_respects_policy() {
    let provider = FakeGitHubSourceProvider;
    let policy = GitHubSourcePolicy::default();
    assert!(provider
        .read(&policy, REPO, "rfcs/0003-github-source-access-node.md")
        .unwrap()
        .contains("fake-github-source"));
    assert!(provider.read(&policy, REPO, ".env").is_err());
}

#[test]
fn audit_input_never_carries_denied_content() {
    let allowed = SourceAuditInput::allowed(REPO, COMMIT, "README.md", HASH_A);
    let denied = SourceAuditInput::denied(REPO, COMMIT, ".env");
    assert_eq!(allowed.outcome, "allowed");
    assert_eq!(denied.outcome, "denied");
    assert!(denied.content_hash.is_none());
}
