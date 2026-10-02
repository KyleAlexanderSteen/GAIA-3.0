# Security policy

## Report a vulnerability

**Do not** open a public GitHub issue for a security defect.

Until the Foundation mailbox exists, email the repository owner through GitHub's private vulnerability reporting:

1. Open https://github.com/R0GV3TheAvatar/GAIA-3.0/security/advisories/new
2. Include impact, affected tree (`gaia-kernel`, `gaia-sdk`, spec, …), and a reproduction if you have one.

If private advisories are unavailable, open a **minimal** issue titled `[SECURITY] request private channel` with **no exploit details**.

## Security Response Team (SRT)

Interim SRT: repository maintainers. Target first response: 5 business days. Critical identity / signing defects: 72 hours.

## Scope

In scope: syscall surface, AIP Manifest validation, SDK crypto stubs, CI secrets, identity documents.

Out of scope until Phase 1 ships: claims about kernel memory safety, FUSE SFS, or production audit-log durability.

## Disclosure

We prefer coordinated disclosure. Fixes land on `main` with a note in the advisory. Please do not publish working exploits before a patch or 90 days, whichever comes first.
