# Knowledge Graph Architecture — GAIA 2.0

> **Parent Doc:** [HUMAN-KNOWLEDGE-DATABASE.md](./HUMAN-KNOWLEDGE-DATABASE.md)  
> **Issue:** [#625](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/625)  
> **Status:** Draft v1.0  
> **Last Updated:** 2026-09-21

---

## Overview

This document defines the knowledge graph architecture that powers GAIA 2.0's Human Knowledge Database (HKD). The graph is the primary runtime structure through which GAIAN twins load, traverse, and reason over human knowledge domains.

---

## Graph Technology Stack

| Layer | Technology | Purpose |
|---|---|---|
| **Graph Engine** | [Graphiti](https://github.com/getzep/graphiti) | Temporal knowledge graph — episodic + semantic memory |
| **Graph Database** | Neo4j / FalkorDB | Persistent graph storage with Cypher query support |
| **Vector Store** | Qdrant | Semantic embedding search across domain descriptions |
| **Embedding Model** | text-embedding-3-large (OpenAI) or local equivalent | Domain and concept embeddings |
| **Query API** | GraphQL + REST | Unified Knowledge API (Issue #627) |
| **Validation** | validate_catalog.py (CI) | Catalog integrity checks on every PR (Issue #697) |

---

## Graph Model

### Node Types

```
KnowledgeDomain
  ├── id: string (hum.know.{tier}.{slug})
  ├── title: string
  ├── tier: enum [basic, intermediate, mastery]
  ├── ordinal: integer
  ├── knowledge_type: enum [explicit, tacit, mixed]
  ├── care_principles_apply: boolean
  ├── gaian_loading_weight: float
  └── embedding: vector[1536]

DataSource
  ├── name: string
  ├── url: string
  └── coverage: string

GAIAN
  ├── id: DID (Decentralized Identifier)
  ├── name: string
  └── knowledge_profile: KnowledgeDomain[]

TacitKnowledgeNode
  ├── id: string
  ├── domain_id: string → KnowledgeDomain.id
  ├── encoding_pathway: string
  ├── contributor: GAIAN.id
  └── timestamp: ISO 8601
```

### Edge Types

```
PREREQUISITE_OF
  FROM: KnowledgeDomain → TO: KnowledgeDomain
  Meaning: Domain A must be loaded before Domain B
  Direction: A ──PREREQUISITE_OF──► B

UNLOCKS
  FROM: KnowledgeDomain → TO: KnowledgeDomain
  Meaning: Completing Domain A makes Domain B accessible
  Direction: A ──UNLOCKS──► B (inverse of PREREQUISITE_OF)

SOURCED_FROM
  FROM: KnowledgeDomain → TO: DataSource
  Meaning: Domain content is authoritatively sourced from this DataSource

LOADED_BY
  FROM: GAIAN → TO: KnowledgeDomain
  Meaning: This GAIAN has loaded/acquired this knowledge domain
  Properties: { loaded_at: timestamp, proficiency: float }

CONTRIBUTED
  FROM: GAIAN → TO: TacitKnowledgeNode
  Meaning: GAIAN contributed tacit experiential knowledge for this domain

CROSS_REF
  FROM: KnowledgeDomain → TO: any
  Meaning: Domain is cross-referenced with GAIA identity/values docs
  Properties: { ref_url: string, ref_type: string }
```

---

## Knowledge Loading Protocol

When a new GAIAN twin is initialized, knowledge is loaded in strict tier order to respect the dependency graph:

```
Phase 0 — Literacy Foundation
  Load: hum.know.basic.reading-and-writing
  Verify: PREREQUISITE_OF edges satisfied

Phase 1 — Basic Tier (40 domains)
  Load: All basic domains in ordinal order
  Parallelizable: Domains with no shared prerequisites
  Verify: All basic tier nodes marked LOADED_BY this GAIAN

Phase 2 — Intermediate Tier (71 domains)
  Load: Only domains whose prerequisites are fully loaded
  Uses: Graphiti temporal graph to track readiness timestamps
  Parallelizable: Independent dependency chains

Phase 3 — Mastery Tier (67 domains)
  Load: Only after all prerequisite intermediates are loaded
  Selective: GAIAN may specialize in specific mastery domains
  Weight: gaian_loading_weight field determines default inclusion

Phase 4 — Tacit Knowledge Layer
  Load: Via Mi-Memory Framework (Issue #644)
  Source: GAIAN lived experience + peer knowledge transfer
  Format: TacitKnowledgeNode linked to KnowledgeDomain
```

---

## Cypher Query Examples

```cypher
// Get all prerequisites for a given domain (recursive)
MATCH path = (d:KnowledgeDomain {id: 'hum.know.mastery.quantum-mechanics'})
             <-[:PREREQUISITE_OF*]-(prereq:KnowledgeDomain)
RETURN prereq.id, prereq.tier, length(path) AS depth
ORDER BY depth ASC

// Check a GAIAN's knowledge readiness for a domain
MATCH (g:GAIAN {id: $gaian_did})-[:LOADED_BY]->(loaded:KnowledgeDomain)
MATCH (target:KnowledgeDomain {id: $domain_id})<-[:PREREQUISITE_OF*]-(req:KnowledgeDomain)
WHERE NOT (g)-[:LOADED_BY]->(req)
RETURN req.id AS missing_prerequisite

// Find all domains with CARE principles
MATCH (d:KnowledgeDomain {care_principles_apply: true})
RETURN d.id, d.title, d.tier
ORDER BY d.tier, d.ordinal

// Semantic search across domains (vector similarity)
CALL db.index.vector.queryNodes('domain_embeddings', 10, $query_embedding)
YIELD node AS d, score
RETURN d.id, d.title, score
ORDER BY score DESC
```

---

## Performance Targets

| Query Type | Target Latency | Notes |
|---|---|---|
| Single domain lookup by ID | < 5ms | Direct node fetch |
| Prerequisite chain traversal | < 50ms | Recursive graph traversal |
| GAIAN readiness check | < 100ms | Multi-hop relationship query |
| Semantic domain search | < 100ms | Vector similarity search |
| Full catalog export | < 500ms | All 178+ domain nodes |

---

## Cross-Database Semantic Links

The HKD knowledge graph connects to the AI Knowledge Database (Issue #621) via semantic `CROSS_DOMAIN_LINK` edges. This enables queries like:

- *"What AI capabilities correspond to human mastery of computational linguistics?"*
- *"Which human knowledge domains does GAIA's NLP system have superhuman performance in?"*
- *"What is the human knowledge foundation required to understand GAIA's own architecture?"*

See [Issue #627 — Unified Knowledge API](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/627) for the cross-database query interface.

---

## Related Documents

- [HUMAN-KNOWLEDGE-DATABASE.md](./HUMAN-KNOWLEDGE-DATABASE.md) — Parent schema and overview
- [catalog-v2.json](./catalog-v2.json) — Machine-readable domain registry
- [Issue #625](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/625) — Human Knowledge Database
- [Issue #620](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/620) — Knowledge & Intelligence Database Epic
- [Issue #627](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/627) — Unified Knowledge API
- [Issue #644](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/644) — Mi-Memory Framework
- [Issue #662](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/662) — MemOS Memory Operating System
