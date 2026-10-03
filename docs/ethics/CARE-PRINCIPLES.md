# CARE Principles for Indigenous Data Sovereignty — GAIA 2.0

> **Issue:** [#676](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/676)  
> **Parent:** [#673 — Governance, Ethics, Funding & Open-Source Foundations](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/673)  
> **Status:** Policy v1.0 — Ratification Pending (Council + Indigenous Seat Holders)  
> **Last Updated:** 2026-09-21  
> **Reference:** GAIA 2.0 + GAIAN 2.0 The CARE Principles.md (source document)

---

## Statement of Commitment

The CARE Principles for Indigenous Data Governance — **Collective Benefit, Authority to Control, Responsibility, Ethics** — are not optional for GAIA 2.0. They are foundational.

Indigenous communities hold millennia of ecological, biological, cultural, and cosmological knowledge that is irreplaceable and essential to understanding Earth's living systems. GAIA 2.0 is built in **partnership with** Indigenous peoples, not in extraction from them. No Traditional Ecological Knowledge (TEK) enters any GAIA 2.0 system without explicit community consent, and communities retain permanent authority to withdraw, restrict, or delete their knowledge at any time.

This policy is binding on all GAIA 2.0 contributors, operators, and GAIAN twin deployments.

---

## The Four CARE Principles

### C — Collective Benefit

> *Data ecosystems should be designed and function in ways that enable Indigenous Peoples to derive benefit from the data.*

**GAIA 2.0 Implementation:**

- **Economic benefit:** Indigenous communities receive direct economic benefit from any commercial use of their TEK within GAIA 2.0 systems
- **Attribution:** All TEK is attributed to the contributing community in every context where it appears
- **AI Dividend:** Indigenous contributor communities receive an **enhanced AI Dividend share** — a premium tier above the standard dividend distributed to all GAIA 2.0 contributors
- **Free GAIAN access:** Any Indigenous community that contributes TEK receives free, locally hosted, community-controlled GAIAN twin deployment for their members
- **Capacity building:** GAIA 2.0 commits resources to Indigenous-led data literacy and AI literacy programs

### A — Authority to Control

> *Indigenous Peoples' rights and interests in Indigenous data must be recognised and their authority to control such data must be acknowledged.*

**GAIA 2.0 Implementation:**

- **Ownership tagging:** All Indigenous ecological and cultural knowledge held in GAIA 2.0 is tagged with the owning community's identity and consent metadata (see [TEK-DATA-GOVERNANCE.md](./TEK-DATA-GOVERNANCE.md))
- **Consent required:** Access to any tagged TEK node requires explicit, current consent from the relevant community
- **Right to withdraw:** Communities can withdraw consent and trigger full deletion of their data from all GAIA 2.0 systems within **24 hours** via the Community Consent API
- **Local Contexts labels:** The [TK (Traditional Knowledge) and BC (Biocultural) label system](https://localcontexts.org/) is integrated into all GAIA 2.0 data systems (see [LOCAL-CONTEXTS-INTEGRATION.md](./LOCAL-CONTEXTS-INTEGRATION.md))
- **FPIC:** Free, Prior, and Informed Consent is required before any TEK integration begins (see [FPIC-PROTOCOL.md](./FPIC-PROTOCOL.md))
- **Sacred knowledge:** Certain knowledge must never be digitised. Communities designate sacred knowledge; GAIA 2.0 enforces the never-share flag at all system layers

### R — Responsibility

> *Those working with Indigenous data have a responsibility to share how those data are used to support Indigenous Peoples' self-determination and collective benefit.*

**GAIA 2.0 Implementation:**

- **Annual TEK Transparency Report:** Published every year — how Indigenous knowledge was accessed, in what contexts, by which GAIAN twins or systems, and what benefit was delivered
- **Real-time data use notifications:** Communities are notified whenever their knowledge is accessed in a GAIA 2.0 system (configurable: summary digest or real-time alerts)
- **Benefit-sharing agreements:** Formal legal agreements (see [BENEFIT-SHARING-TEMPLATE.md](./BENEFIT-SHARING-TEMPLATE.md)) are executed before any TEK integration
- **Community veto:** Any community can veto a specific use of their knowledge, even after initial consent, with immediate effect
- **Audit trail:** All TEK access is logged with immutable timestamps; communities can request full access logs at any time

### E — Ethics

> *Indigenous Peoples' rights and wellbeing should be the primary concern at all stages of the data life cycle.*

**GAIA 2.0 Implementation:**

- **Indigenous ethics review:** All TEK integrations are reviewed by Indigenous ethics advisors before approval; no majority-culture override is permitted
- **Do no harm:** No TEK is used in ways that could harm the contributing community — economic, cultural, spiritual, or political harm
- **Cultural protocols:** Deep respect for sacred knowledge, ceremonial knowledge, and gender-restricted knowledge; GAIA 2.0 never digitises knowledge the community has designated as non-digitisable
- **Long-term relationship:** GAIA 2.0 commits to multi-generational relationships with partner communities — not one-time data transactions
- **Intergenerational consideration:** Decisions about TEK use consider impact on future generations of the contributing community

---

## Indigenous Knowledge Domains in GAIA 2.0

The following categories of Traditional Ecological Knowledge are relevant to GAIA 2.0's knowledge systems. All are subject to this policy and the TEK data governance framework.

```
TEK CATEGORIES RELEVANT TO GAIA 2.0:
  ├── Species knowledge
  │     Local names, behaviour, habitat, seasonal patterns, ecological roles
  ├── Ecological relationships
  │     Food webs, mutualism, indicator species, keystone species knowledge
  ├── Land management
  │     Controlled burning, water management, soil practices, permaculture
  ├── Climate indicators
  │     Traditional weather knowledge, seasonal calendars, phenological indicators
  ├── Language
  │     Indigenous species names → multilingual GBIF + Biological Layer integration
  └── Sacred sites
        Locations NOT to be shared, mapped, or made publicly accessible
```

**Knowledge database integration:** Domains with `care_principles_apply: true` in `catalog-v2.json` are governed by this policy. See [docs/knowledge/catalog-v2.json](../knowledge/catalog-v2.json) for the full list.

---

## Partner Networks

GAIA 2.0 is building formal relationships with the following organisations as part of CARE Principles implementation:

| Organisation | Role | Status |
|---|---|---|
| [Local Contexts](https://localcontexts.org/) | TK + BC label system for TEK data governance | Integration in progress |
| [GIDA — Global Indigenous Data Alliance](https://www.gida-global.org/) | CARE Principles stewards; governance advisory | Engagement pending |
| [FNIGC — First Nations Information Governance Centre](https://fnigc.ca/) | Canada — First Nations data sovereignty | Engagement pending |
| [AIATSIS](https://aiatsis.gov.au/) | Australia — Aboriginal & Torres Strait Islander Studies | Engagement pending |
| [Amazon Sacred Headwaters Alliance](https://sacredheadwaters.org/) | Amazon basin Indigenous communities | Engagement pending |
| [Pacific Community (SPC)](https://www.spc.int/) | Pacific Islands Indigenous data governance | Engagement pending |
| [UNPFII](https://www.un.org/development/desa/indigenouspeoples/unpfii-sessions-2.html) | UN Permanent Forum on Indigenous Issues | Dialogue pending |

---

## Governance & Ratification

This policy takes effect upon ratification by the GAIA 2.0 Multi-Stakeholder Council ([Issue #675](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/675)), which must include at minimum **two Indigenous seat holders** from distinct geographic regions before any TEK integration may begin.

Amendments to this policy require:
1. Proposal from any Council member or partner community
2. 30-day community comment period
3. Approval by simple majority of Council including at least one Indigenous seat holder
4. No amendment may reduce the rights or protections afforded to Indigenous communities

---

## Related Documents

| Document | Purpose |
|---|---|
| [FPIC-PROTOCOL.md](./FPIC-PROTOCOL.md) | Free, Prior, and Informed Consent standard process |
| [LOCAL-CONTEXTS-INTEGRATION.md](./LOCAL-CONTEXTS-INTEGRATION.md) | TK + BC label system integration |
| [BENEFIT-SHARING-TEMPLATE.md](./BENEFIT-SHARING-TEMPLATE.md) | Legal template for community benefit-sharing agreements |
| [TEK-DATA-GOVERNANCE.md](./TEK-DATA-GOVERNANCE.md) | Technical TEK tagging, consent API, and sacred knowledge enforcement |
| [docs/knowledge/catalog-v2.json](../knowledge/catalog-v2.json) | Domains with `care_principles_apply: true` |
| [Issue #675](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/675) | Governance Framework — Multi-Stakeholder Council |
| [Issue #677](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/677) | Funding Strategy & AI Dividend Model |
