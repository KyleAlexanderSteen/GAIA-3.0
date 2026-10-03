# Traditional Ecological Knowledge Data Governance — GAIA 2.0

> **Parent:** [CARE-PRINCIPLES.md](./CARE-PRINCIPLES.md)  
> **Issue:** [#676](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/676)  
> **Status:** Technical Spec v1.0  
> **Last Updated:** 2026-09-21

---

## Overview

This document defines the technical architecture for how Traditional Ecological Knowledge (TEK) is tagged, stored, governed, and accessed in GAIA 2.0 systems. It is the implementation spec for the Authority to Control and Responsibility principles in [CARE-PRINCIPLES.md](./CARE-PRINCIPLES.md).

---

## TEK Node Schema

Every TEK data node in GAIA 2.0 carries the following mandatory governance fields in addition to its content fields:

```json
{
  "id": "tek.{region}.{community-slug}.{knowledge-slug}",
  "title": "string",
  "community": "string — community's preferred name",
  "community_id": "string — GIDA-registered community identifier (where available)",
  "region": "string — geographic region",
  "care_principles_apply": true,
  "local_contexts": {
    "project_id": "string — Local Contexts Hub project ID",
    "tk_labels": ["TK-A", "TK-NC"],
    "bc_labels": ["BC-P"],
    "label_url": "string",
    "community_notice": "string — plain-language notice from community"
  },
  "fpic_completed": true,
  "benefit_sharing_agreement_id": "string — agreement reference",
  "consent_status": "active | suspended | withdrawn",
  "consent_granted_at": "ISO 8601 date",
  "consent_expires_at": "ISO 8601 date | null",
  "sacred": false,
  "access_tier": "public | consented-research | community-only | sacred-blocked",
  "knowledge_type": "tacit | explicit | mixed",
  "tacit_encoding_pathway": "string | null",
  "wikidata_id": "string | null",
  "gbif_taxon_id": "integer | null",
  "version": "semver",
  "last_validated": "ISO 8601 date",
  "created_at": "ISO 8601 datetime",
  "updated_at": "ISO 8601 datetime"
}
```

---

## Access Tier Definitions

| Tier | Code | Who Can Access | Enforcement |
|---|---|---|---|
| Public (with attribution) | `public` | Any GAIAN or API consumer — with mandatory community attribution | Attribution check at API layer |
| Consented Research | `consented-research` | Authenticated researchers; non-commercial use; active benefit-sharing agreement | Auth + agreement status check |
| Community Only | `community-only` | Verified members of the contributing community via authenticated GAIAN | Community membership verification |
| Sacred — Blocked | `sacred-blocked` | No one — content is null in all responses | Hard null; access attempt logged + community notified |

---

## Community Consent API

The Community Consent API enables partner communities to manage their TEK access programmatically — granting, suspending, or withdrawing consent; updating access tiers; and triggering data deletion — without depending on GAIA 2.0 staff.

### Endpoints

```
GET    /consent/community/{community_id}/nodes
         → List all TEK nodes for this community with current consent status

PATCH  /consent/node/{node_id}
         → Update consent_status, access_tier, or sacred flag for a node
         Body: { "consent_status": "suspended", "reason": "string" }

DELETE /consent/node/{node_id}
         → Initiate full deletion of this TEK node from all GAIA 2.0 systems
         → SLA: complete deletion within 24 hours
         → Deletion confirmed via webhook to community endpoint

POST   /consent/community/{community_id}/withdraw-all
         → Withdraw consent for ALL community TEK nodes
         → Triggers full deletion of all community TEK within 24 hours

GET    /consent/community/{community_id}/access-log
         → Full audit log of all accesses to community TEK nodes
         → Filterable by date range, accessing system, GAIAN ID

GET    /consent/community/{community_id}/benefit-report
         → Current benefit-sharing ledger: AI Dividend accrued, payments made
```

### Authentication

Community Consent API access is authenticated via community-specific API keys, issued during the FPIC process and managed by the community's designated liaison. Keys can be rotated at any time. GAIA 2.0 never holds backup copies of community API keys.

### Deletion SLA

Upon receiving a deletion request (individual node or withdraw-all), GAIA 2.0 systems must:

1. Remove node content from all live caches — **within 1 hour**
2. Remove node from knowledge graph (Graphiti / Neo4j) — **within 4 hours**
3. Remove node from vector store (Qdrant) — **within 4 hours**
4. Remove node from cold storage / backups — **within 24 hours**
5. Send deletion confirmation webhook to community endpoint — **within 24 hours**
6. Include deletion in next Annual TEK Transparency Report

---

## Sacred Knowledge Enforcement

Knowledge designated as `sacred: true` or `access_tier: sacred-blocked` is enforced at multiple independent layers:

```
Layer 1 — Database: Node content stored as null; only governance metadata retained
Layer 2 — API: Node returns 403 with care notice; content field is absent
Layer 3 — GAIAN twin: Knowledge domain blocked from loading; GAIAN cannot surface it
Layer 4 — Vector store: No embedding created; semantic search cannot surface sacred nodes
Layer 5 — Audit: Every access attempt (even 403) logged and batched for community notification
```

Sacred knowledge designation is **community-only** — GAIA 2.0 staff cannot override or remove a sacred designation. Only the contributing community can change a sacred designation, through the Community Consent API.

---

## TEK Tagging in catalog-v2.json

All knowledge domains in the HKD catalog that involve Indigenous or traditional knowledge carry `"care_principles_apply": true`. These domains are the **entry points** into the TEK data governance system — individual TEK nodes are linked from these domain entries.

Domains currently flagged:
- `hum.know.basic.basic-religion-and-spirituality`
- `hum.know.basic.basic-mythology`
- `hum.know.basic.basic-agriculture`
- `hum.know.intermediate.comparative-religion`
- `hum.know.mastery.advanced-anthropology`

As TEK nodes are contributed and FPIC-completed, they are linked to their parent domain via the Graphiti knowledge graph using the `SOURCED_FROM` edge with `care_principles_apply: true` metadata.

---

## Annual TEK Transparency Report

Published annually, the TEK Transparency Report covers:

1. **Access summary:** Number of accesses per community, per TEK node, per GAIAN twin type
2. **Consent status changes:** Nodes withdrawn, suspended, or newly activated in the reporting year
3. **Benefit delivery:** AI Dividend amounts paid to each community; GAIAN deployments provided
4. **Sacred knowledge integrity:** Confirmation that no sacred-blocked nodes were accessed
5. **FPIC pipeline:** New community engagements initiated, completed, and pending
6. **Incidents:** Any unauthorized access attempts or system failures affecting TEK governance

The report is published publicly and sent directly to each partner community in their preferred language and format.

---

## Related Documents

- [CARE-PRINCIPLES.md](./CARE-PRINCIPLES.md) — Policy
- [FPIC-PROTOCOL.md](./FPIC-PROTOCOL.md) — Consent process
- [LOCAL-CONTEXTS-INTEGRATION.md](./LOCAL-CONTEXTS-INTEGRATION.md) — Label system
- [BENEFIT-SHARING-TEMPLATE.md](./BENEFIT-SHARING-TEMPLATE.md) — Legal template
- [docs/knowledge/catalog-v2.json](../knowledge/catalog-v2.json) — Domains with `care_principles_apply: true`
- [Issue #661](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/661) — Biological Layer (primary TEK consumer)
- [Issue #677](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/677) — AI Dividend Model
