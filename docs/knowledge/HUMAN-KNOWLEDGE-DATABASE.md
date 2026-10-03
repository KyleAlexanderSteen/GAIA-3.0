# Human Knowledge Database — GAIA 2.0

> **Issue:** [#625](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/625)  
> **Epic:** [#620 — Knowledge & Intelligence Database System](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/620)  
> **Status:** Draft — Schema v1.0  
> **Last Updated:** 2026-09-21

---

## Overview

The Human Knowledge Database (HKD) is GAIA 2.0's canonical registry of all documented human knowledge domains. It serves as the structured backbone for GAIAN twin knowledge loading, RAG retrieval, cross-domain reasoning, and the Mi-Memory Framework's tacit knowledge encoding layer.

Unlike the AI Knowledge Database (which is 100% explicit), human knowledge is approximately **85% tacit** (embodied, experiential, lived) and **15% explicit** (documented, transferable via text). The HKD captures the explicit 15% in structured, queryable form and creates pathways for encoding the tacit 85% through GAIAN profiles.

---

## Architecture

### Knowledge Tier Model

The HKD organizes all human knowledge into three progressive tiers:

```
TIER 0 — LITERACY FOUNDATION
  └── Reading & Writing (prerequisite meta-skill for all knowledge)

TIER 1 — BASIC (40 domains)
  └── Foundational knowledge every human and GAIAN twin must possess
  └── ID namespace: hum.know.basic.*

TIER 2 — INTERMEDIATE (71 domains)
  └── Academic and applied knowledge requiring basic tier as prerequisite
  └── ID namespace: hum.know.intermediate.*

TIER 3 — MASTERY (67 domains)
  └── Expert, frontier, and specialist knowledge
  └── ID namespace: hum.know.mastery.*

TOTAL: 178 knowledge domains (+ 3 proposed additions — see Issue #698)
```

### Domain Identifier Schema

Every knowledge domain follows this canonical ID format:

```
{audience}.{plane}.{tier}.{slug}
```

| Field | Value Space | Example |
|---|---|---|
| `audience` | `hum` (human), `ai` (AI system) | `hum` |
| `plane` | `know` (knowledge), `skill` (skills) | `know` |
| `tier` | `basic`, `intermediate`, `mastery` | `basic` |
| `slug` | kebab-case domain name | `basic-mathematics` |

**Full example:** `hum.know.basic.basic-mathematics`

---

## Domain Node Schema

Each domain in the HKD is represented as a structured node with the following schema:

```json
{
  "id": "hum.know.basic.basic-mathematics",
  "title": "Basic Mathematics",
  "slug": "basic-mathematics",
  "tier": "basic",
  "ordinal": 2,
  "audience": "human",
  "plane": "knowledge",
  "description": "Foundational arithmetic, number systems, and quantitative reasoning.",
  "prerequisites": ["hum.know.basic.reading-and-writing"],
  "unlocks": ["hum.know.intermediate.algebra-and-geometry", "hum.know.intermediate.statistics-and-probability"],
  "authoritative_sources": [
    { "name": "Wikidata", "id": "Q395", "url": "https://www.wikidata.org/wiki/Q395" },
    { "name": "arXiv", "category": "math.HO" }
  ],
  "knowledge_type": "explicit",
  "tacit_encoding_pathway": null,
  "care_principles_apply": false,
  "gaian_loading_weight": 1.0,
  "version": "1.0.0",
  "last_validated": "2026-09-21"
}
```

### Schema Field Reference

| Field | Type | Description |
|---|---|---|
| `id` | `string` | Canonical domain ID (see ID schema above) |
| `title` | `string` | Human-readable domain name |
| `slug` | `string` | kebab-case identifier for file/URL routing |
| `tier` | `enum` | `basic` \| `intermediate` \| `mastery` |
| `ordinal` | `integer` | Sort order within tier |
| `audience` | `enum` | `human` \| `ai` |
| `plane` | `enum` | `knowledge` \| `skills` |
| `description` | `string` | One-sentence domain summary |
| `prerequisites` | `string[]` | Domain IDs that must be loaded before this one |
| `unlocks` | `string[]` | Domain IDs this domain enables |
| `authoritative_sources` | `object[]` | Wikidata, arXiv, PubMed, or other authoritative links |
| `knowledge_type` | `enum` | `explicit` \| `tacit` \| `mixed` |
| `tacit_encoding_pathway` | `string\|null` | Mi-Memory pathway ID for tacit knowledge encoding |
| `care_principles_apply` | `boolean` | Whether CARE Principles apply (indigenous/traditional knowledge) |
| `gaian_loading_weight` | `float` | Relative weight for GAIAN twin knowledge loading (0.0–1.0) |
| `version` | `semver` | Schema version of this domain entry |
| `last_validated` | `ISO 8601` | Date domain entry was last reviewed |

---

## Knowledge Graph Architecture

The HKD is not just a flat list — it is a **directed acyclic graph (DAG)** where domains are nodes and `prerequisites`/`unlocks` relationships are edges. This enables:

- **Progressive knowledge loading** for GAIAN twins (basic → intermediate → mastery)
- **Dependency resolution** before activating a knowledge domain
- **Gap analysis** — identify which intermediate domains a GAIAN is missing given their basic tier completeness
- **Cross-database linking** — semantic edges connect HKD nodes to AI Knowledge Database nodes

### Graph Technology

The GAIA 2.0 HKD knowledge graph is powered by **Graphiti** (temporal knowledge graph framework), enabling:
- Timestamped knowledge acquisition per GAIAN twin
- Episodic + semantic memory integration
- Sub-100ms query response for knowledge graph traversal

### Graph Query Examples

```
# What domains does a GAIAN need before learning Quantum Mechanics?
DEPENDENCIES(hum.know.mastery.quantum-mechanics)
→ [hum.know.intermediate.advanced-physics, hum.know.intermediate.calculus,
   hum.know.intermediate.algebra-and-geometry, hum.know.basic.basic-physics,
   hum.know.basic.basic-mathematics]

# What mastery domains does a GAIAN unlock after completing all intermediates?
UNLOCKS_AT_TIER(hum.know.intermediate.*, mastery)
→ [67 mastery domain nodes]

# Which domains require CARE Principles enforcement?
FILTER(care_principles_apply == true)
→ [indigenous knowledge nodes]
```

---

## Data Sources & Integrations

| Source | Coverage | Integration Method |
|---|---|---|
| **Wikidata** | 16B+ triples, all domains | SPARQL endpoint + entity linking |
| **arXiv** | Science, math, CS, physics | API + semantic chunking |
| **PubMed / MEDLINE** | Medicine, biology, health | E-utilities API |
| **Open Library / Project Gutenberg** | Literature, humanities | OAI-PMH harvest |
| **Stanford Encyclopedia of Philosophy** | Philosophy, ethics, logic | Structured scrape + SEP API |
| **GBIF** | Ecology, biology, environmental science | REST API (see Issue #658) |
| **GAIAN Mi-Memory Profiles** | Tacit & experiential knowledge | Mi-Memory Framework (Issue #644) |

---

## CARE Principles for Indigenous & Traditional Knowledge

All indigenous, traditional, and community-held knowledge domains must comply with the **CARE Principles for Indigenous Data Governance** (Collective Benefit, Authority to Control, Responsibility, Ethics). See [Issue #676](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/676) for full implementation.

- Domains with `care_principles_apply: true` require explicit community consent before loading into any GAIAN twin
- No indigenous knowledge nodes may be commercially exploited without community benefit-sharing agreements
- Oral traditions and non-documented knowledge are encoded only via GAIAN opt-in pathways

---

## Tacit Knowledge Encoding

The 85% of human knowledge that is tacit (skills, intuition, embodied experience, cultural practice) is encoded through the **Mi-Memory Framework** (Issue #644). Each GAIAN twin builds a personal tacit knowledge layer through:

1. **Lived experience logging** — GAIAN records experiences that encode tacit skills
2. **Peer knowledge transfer** — structured sharing between GAIAN twins
3. **Expert elicitation** — structured interviews with human domain experts
4. **Skill demonstration encoding** — video/sensor capture of physical skills

Tacit knowledge nodes carry a `tacit_encoding_pathway` field linking to the Mi-Memory pathway responsible for encoding that domain.

---

## Acceptance Criteria (from Issue #625)

- [ ] All 178 domains have fully populated schema entries in `catalog-v2.json`
- [ ] `prerequisites` and `unlocks` edges defined for all intermediate and mastery domains
- [ ] Wikidata entity IDs linked for all domains where applicable
- [ ] Graphiti knowledge graph serves domain queries with <100ms response
- [ ] CARE Principles enforced for all indigenous knowledge nodes
- [ ] GAIAN-contributed tacit knowledge accepted and linked into graph via Mi-Memory
- [ ] Cross-database semantic search works across Human + AI Knowledge DBs (Issue #627)
- [ ] `validate_catalog.py` CI check passes on all catalog entries (Issue #697)
- [ ] 3 new domains added: `basic-engineering`, `basic-programming`, `basic-data-literacy` (Issue #698)

---

## Related Issues

| Issue | Title |
|---|---|
| [#620](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/620) | [EPIC] Knowledge & Intelligence Database System |
| [#621](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/621) | AI Knowledge Database |
| [#626](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/626) | Human Skills & Superpowers Database |
| [#627](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/627) | Unified Knowledge API |
| [#628](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/628) | Hallucination Detection & Knowledge Quality |
| [#644](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/644) | Mi-Memory Framework |
| [#676](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/676) | CARE Principles Implementation |
| [#693](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/693) | Migrate 3-tier knowledge taxonomy |
| [#695](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/695) | Integrate catalog.json as knowledge registry |
| [#697](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/697) | Port validate_catalog.py as CI lint check |
| [#698](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/698) | Add missing foundational domains |
| [#701](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/701) | Cross-reference futurism/transhumanism with identity docs |
