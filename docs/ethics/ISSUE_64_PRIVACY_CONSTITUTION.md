# GAIAN Privacy Constitution and age-gate

**Author:** Kyle Steen / Kyle Alexander Steen the Alchemist  
**Attribution:** immutable  
**GitHub:** #64 (parent #58)  
**Related:** [Issue 221 constitution](ISSUE_221_CONSTITUTION.md)

This is law. It does not study the Wizard. Default is refuse.

## Ten principles

1. **Consent.** Granular, current, revocable. No implied ingest.
2. **Local-default.** Data stays on the Gaian's machine unless they share it.
3. **Biometric sovereignty.** Face, body, voice, and health signals are not training fuel.
4. **User-held keys.** The Gaian holds keys. GAIA does not keep a silent copy.
5. **Deletion.** Revoke means cryptographic erasure or equivalent wipe, not a hide flag.
6. **Transparency.** Purpose, retention, and processors are disclosed in plain language.
7. **Purpose limitation.** Collected for X cannot be reused for Y without a new grant.
8. **Non-weaponization.** No targeting, scoring, or sale to insurers, employers, or advertisers.
9. **Equity.** Protections do not weaken by class, race, disability, or nation.
10. **Child protection.** Under 16: guardian consent, no profiling, no health-twin learning, no Phase 3 personality learning.

## Consent catalog (SDK enum)

Machine-readable file: `gaia-sdk/consent_scopes.json`

- `face`
- `body`
- `voice`
- `health`
- `memory`
- `agent`
- `earth_twin_share`

## Age-gate

- Under 16 requires verifiable guardian consent.
- No behavioral profiling of children.
- No health-twin learning on children.
- Child path cannot enable Phase 3 personality learning.
- Aligns with issue 221: no Level 2+ agency, no ambient listen.

## Forbidden

- Third-party likeness without that person's consent
- Hidden copies
- Insurer or employer health export

## README link

Link this document from the repository README:
`docs/ethics/ISSUE_64_PRIVACY_CONSTITUTION.md`
