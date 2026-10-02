//! Skills first cuts (#107–#120). Local-default. No child EI. No v1.0.

mod assess;
mod bands;
mod develop;
mod graph;
mod matching;
mod overlay;
mod path;
mod profile;
mod schema;
mod vault;

pub use assess::{hidden_profile_api, Badge, Session};
pub use bands::{ascendence_eligible, ascendence_self_claim, catalog, challenge_is_grant, declare_is_holding, entanglement_is_both, grants_anything, knowing_is_having, magic_is_meta_band, meta_rows, synthesis_into_magic_band, Band, BandRow, Eligibility, Witness};
pub use develop::{develop, DevPath};
pub use graph::SkillGraph;
pub use matching::{global_profile_dump, skills_v1_tagged, tek_skill, SkillCard};
pub use overlay::{
    bessi_domains, digcomp_areas, realm_stubs, research_realm_bind, research_realms, wef_2025_essay,
    wef_resolves, wef_top10,
};
pub use path::novice_public_speaking;
pub use profile::SkillProfile;
pub use schema::{active_listening, SkillNode, REALMS};
pub use vault::VaultProfile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillError {
    ChildEiBlocked,
    SyncDenied,
    UnknownSkill,
    AmbientDenied,
    NoGrant,
    ChildRank,
    ClinicalCert,
    Unpublished,
}
