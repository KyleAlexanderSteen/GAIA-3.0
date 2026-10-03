# Local Contexts Integration — TK & BC Labels — GAIA 2.0

> **Parent:** [CARE-PRINCIPLES.md](./CARE-PRINCIPLES.md)  
> **Issue:** [#676](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/676)  
> **Status:** Integration Spec v1.0  
> **Last Updated:** 2026-09-21

---

## Overview

[Local Contexts](https://localcontexts.org/) is an international initiative that supports Indigenous communities in managing their intellectual property and cultural heritage through the **Traditional Knowledge (TK) Labels** and **Biocultural (BC) Labels** system. GAIA 2.0 integrates Local Contexts labels as the primary mechanism for communicating Indigenous community rights, permissions, and restrictions on TEK data nodes.

---

## Label Types

### Traditional Knowledge (TK) Labels

TK Labels are used by Indigenous communities to communicate cultural protocols and restrictions on their traditional knowledge.

| Label | Code | Meaning for GAIA 2.0 |
|---|---|---|
| TK Attribution | `TK-A` | Community must be attributed in all uses |
| TK Clan | `TK-CL` | Restricted to specific clan members |
| TK Family | `TK-F` | Restricted to specific family members |
| TK Multiple Communities | `TK-MC` | Shared across multiple communities — all must consent |
| TK Non-Commercial | `TK-NC` | No commercial use permitted |
| TK Seasonal | `TK-S` | Access restricted to specific seasons or ceremonies |
| TK Secret/Sacred | `TK-SS` | Not for public access — GAIA 2.0 enforces hard block |
| TK Men General | `TK-MG` | General access for men in the community |
| TK Women General | `TK-WG` | General access for women in the community |
| TK Men Restricted | `TK-MR` | Restricted to specific men in the community |
| TK Women Restricted | `TK-WR` | Restricted to specific women in the community |
| TK Verified | `TK-V` | Community has verified accuracy of this knowledge |
| TK Non-Verified | `TK-NV` | Community has not verified — treat as unvalidated |
| TK Open to Commercialisation | `TK-OC` | Community permits commercial use with benefit-sharing |

### Biocultural (BC) Labels

BC Labels communicate the biocultural relationships between communities and their lands, waters, and living systems — directly relevant to GAIA 2.0's Biological Layer.

| Label | Code | Meaning for GAIA 2.0 |
|---|---|---|
| BC Provenance | `BC-P` | This community is the source of this biological/ecological knowledge |
| BC Notice | `BC-N` | Notice that this data involves Indigenous biocultural relationships |
| BC Authorization | `BC-AUTH` | Community has authorised this specific use |
| BC Non-Commercial | `BC-NC` | No commercial use of biocultural knowledge |
| BC Research Use | `BC-R` | Permitted for non-commercial research only |
| BC Clan | `BC-CL` | Restricted to specific clan protocols |
| BC Seasonal | `BC-S` | Seasonal or ceremonial access restrictions |
| BC Commercialisation | `BC-C` | Community permits commercialisation with benefit-sharing |

---

## Integration Architecture

### TEK Node Schema Extension

All TEK data nodes in GAIA 2.0 carry a `local_contexts` field in addition to the standard knowledge domain schema:

```json
{
  "id": "tek.amazon.kayapo.fire-management",
  "title": "Kayapó Controlled Fire Management",
  "community": "Kayapó People, Pará, Brazil",
  "care_principles_apply": true,
  "local_contexts": {
    "project_id": "lc-proj-kayapo-gaia2",
    "tk_labels": ["TK-A", "TK-NC", "TK-V"],
    "bc_labels": ["BC-P", "BC-N"],
    "label_url": "https://localcontexts.org/labels/traditional-knowledge/",
    "community_notice": "This knowledge belongs to the Kayapó People. Use requires attribution and community consent."
  },
  "consent_status": "active",
  "consent_granted_at": "2027-01-15",
  "consent_expires_at": null,
  "sacred": false,
  "access_tier": "consented-research"
}
```

### Access Enforcement Rules

GAIA 2.0 enforces Local Contexts labels at the API and GAIAN twin layers:

```
TK-SS (Secret/Sacred):
  → Hard block at all system layers
  → Node exists in registry but content is null
  → Access attempt logged and notified to community

TK-NC / BC-NC (Non-Commercial):
  → Blocked for all commercial GAIAN twin use cases
  → Available for non-commercial research and community use only

TK-CL / TK-F / TK-MR / TK-WR (Restricted):
  → Blocked for all external access
  → Available only to verified community members via authenticated GAIAN

TK-S / BC-S (Seasonal):
  → Time-gated access based on community-defined calendar
  → System checks current date against seasonal access window

All other labels:
  → Attribution required in all outputs
  → Benefit-sharing agreement must be active
  → Access logged for annual transparency report
```

### Local Contexts API Integration

GAIA 2.0 integrates with the [Local Contexts Hub API](https://localcontextshub.org/) to:
- Fetch and sync current label assignments for all partner communities
- Verify label authenticity (labels must originate from the community via the LC Hub)
- Display labels in GAIAN twin interfaces and API responses
- Receive webhook notifications when a community updates their labels

```
Integration endpoint: https://localcontextshub.org/api/v1/
Authentication: API key per GAIA 2.0 operator instance
Sync frequency: Daily + webhook on label change
Label source of truth: Local Contexts Hub (community-controlled)
```

---

## Implementation Checklist

- [ ] Local Contexts Hub API integration (auth, sync, webhook)
- [ ] `local_contexts` field added to TEK node schema
- [ ] Access enforcement rules implemented at API layer
- [ ] Access enforcement rules implemented at GAIAN twin layer
- [ ] TK-SS hard block tested and verified
- [ ] Community label update webhook triggers cache invalidation
- [ ] Labels displayed in all GAIAN interfaces where TEK is surfaced
- [ ] Annual transparency report includes label-level access breakdown
