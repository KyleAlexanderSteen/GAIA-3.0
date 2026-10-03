# GAIA 2.0 Knowledge-Domain Naming Standard

**Status:** Proposed baseline for issue #696  
**Applies to:** GAIA 2.0 knowledge-domain registry, taxonomy migration, documentation paths, URLs, APIs, and compatibility mappings  
**Related issues:** #695, #697, #699, #700, #693

## Purpose

GAIA 2.0 must preserve stable references to knowledge domains while allowing curricula, learning stages, source taxonomies, and editorial names to evolve. A domain is not defined by its current learning tier or by a legacy folder name.

This standard separates:

- **Identity:** stable, machine-readable identifier
- **Slug:** human-readable filesystem and URL form
- **Title:** editorial display name
- **Learning placement:** track- and stage-specific curriculum metadata
- **Legacy compatibility:** aliases and source lineage retained during migration

## Normative language

The keywords **MUST**, **MUST NOT**, **SHOULD**, **SHOULD NOT**, and **MAY** are used as normative requirements.

## Canonical identity

A knowledge-domain record MUST have a stable canonical identifier:

```text
gaia.knowledge.<domain>[.<subdomain>...]
```

Examples:

```text
gaia.knowledge.programming
gaia.knowledge.life-sciences.biology
gaia.knowledge.earth-systems.climate-science
gaia.knowledge.computing.machine-learning
gaia.knowledge.linguistics.computational-linguistics
```

Rules:

- Canonical IDs MUST be lowercase ASCII dotted namespaces.
- Each namespace segment MUST use kebab-case: `a-z`, `0-9`, and single hyphens only.
- Canonical IDs MUST be unique and immutable after public publication.
- Canonical IDs MUST NOT encode learning tier, curriculum stage, repository path, evidence state, access class, or implementation status.
- A rename MUST create an alias or a deprecated record with an explicit successor; it MUST NOT silently replace a published ID.

## Slugs and paths

A domain slug MUST use kebab-case.

```text
✅ programming
✅ reading-and-writing
✅ data-literacy
✅ climate-science
❌ Basic_Programming
❌ basic-programming
❌ programming_basics
```

The canonical domain directory pattern is:

```text
docs/knowledge/domains/<slug>/
```

The registry, not the directory name alone, is authoritative for identity.

## Titles

The `title` field is a human-facing display label in Title Case. It MAY change editorially without changing the canonical ID or slug when the change does not alter the domain's scope.

Examples:

```text
id: gaia.knowledge.programming
slug: programming
title: Programming
```

```text
id: gaia.knowledge.earth-systems.climate-science
slug: climate-science
title: Climate Science
```

## Learning placement is metadata

Basic, intermediate, advanced, and mastery are curriculum placements, not domain identities. A single enduring domain MAY appear in multiple learning tracks and stages.

```json
{
  "id": "gaia.knowledge.programming",
  "slug": "programming",
  "title": "Programming",
  "learning_placements": [
    {
      "track": "general-human-literacy",
      "stage": "foundation",
      "legacy_slug": "basic-programming"
    },
    {
      "track": "computing",
      "stage": "intermediate"
    },
    {
      "track": "computer-science",
      "stage": "mastery"
    }
  ]
}
```

Allowed initial stage values are:

```text
foundation
intermediate
advanced
mastery
```

New stage values require a registry-schema revision and an associated decision record.

## Legacy compatibility and provenance

Legacy folder names MUST be preserved as aliases or source-lineage references during migration. They MUST NOT become new canonical identifiers merely because they existed in a prior repository.

Each migrated record SHOULD include:

```json
{
  "aliases": ["basic-biology", "intermediate-advanced-biology"],
  "legacy": {
    "repository": "KyleAlexanderSteen/GAIA",
    "paths": [
      "docs/knowledge/the-subjects-of-knowledge-for-humans/basic-biology",
      "docs/knowledge/the-subjects-of-knowledge-for-humans/intermediate-advanced-biology"
    ],
    "relabel_map_source": "legacy-main@63dd2cf3"
  }
}
```

A compatibility map MUST be versioned with the registry and MUST preserve enough information to resolve legacy references deterministically.

## Deprecation and replacement

A publicly referenced domain MUST NOT disappear silently. A deprecated record MUST contain:

- `status: "deprecated"`
- `deprecated_at`
- `deprecation_reason`
- `replaced_by` when a successor exists

Example:

```json
{
  "id": "gaia.knowledge.legacy-computing",
  "status": "deprecated",
  "deprecated_at": "2026-09-21",
  "deprecation_reason": "Superseded by domain split.",
  "replaced_by": [
    "gaia.knowledge.programming",
    "gaia.knowledge.computer-science"
  ]
}
```

## Edge cases

### Reading and writing

`reading-and-writing` is retained as a legacy alias during migration. Its long-term classification is intentionally deferred to issue #700. Until that decision is made, it SHOULD be treated as a foundational, cross-cutting domain and MUST NOT be forced into a tier-prefixed canonical identity.

### Legacy use of “advanced”

The word `advanced` in a legacy slug MUST be reviewed before migration:

- If it expresses a learning stage, it belongs in `learning_placements.stage`.
- If it names a distinct discipline, it MAY remain in the canonical title and slug.
- If its meaning is uncertain, retain it as a legacy alias and record the ambiguity for review.

For example, `mastery-advanced-engineering` should normally map to the enduring engineering domain plus a mastery placement, while `computational-linguistics` remains a distinct discipline.

## Examples

| Legacy path or slug | Canonical ID | Canonical slug | Initial placement | Migration treatment |
|---|---|---|---|---|
| `basic-programming` | `gaia.knowledge.programming` | `programming` | `foundation` | Legacy alias |
| `basic-data-literacy` | `gaia.knowledge.data-literacy` | `data-literacy` | `foundation` | New canonical domain proposed by #698 |
| `basic-biology` | `gaia.knowledge.life-sciences.biology` | `biology` | `foundation` | Legacy alias |
| `intermediate-advanced-biology` | `gaia.knowledge.life-sciences.biology` | `biology` | `intermediate` | Legacy alias; review “advanced” as non-canonical stage wording |
| `mastery-advanced-ai-and-machine-learning` | `gaia.knowledge.computing.ai-and-machine-learning` | `ai-and-machine-learning` | `mastery` | Legacy alias; scope review follows #694 |
| `reading-and-writing` | Deferred by #700 | Deferred by #700 | Foundational cross-cutting | Preserve alias and provenance only |

## Required registry fields

Issue #695 MUST define a machine-readable registry schema that includes at least:

```text
id
slug
title
kind
status
aliases
learning_placements
legacy
```

Future schema work SHOULD add governance, provenance, evidence, access-control, and lifecycle fields.

## Validation requirements

Issue #697 MUST enforce at minimum:

- Canonical IDs are unique and conform to this standard.
- Canonical slugs conform to kebab-case and are unique within their registry scope.
- Aliases do not collide with another record's canonical ID or alias unless an explicit compatibility exception exists.
- Every legacy migration record includes a source repository and path.
- Deprecated records state their reason and resolve every listed successor.
- Learning stages use the approved vocabulary.
- No canonical ID contains a legacy tier prefix such as `basic`, `intermediate`, or `mastery` as an identity segment unless an approved exception records why.

## Migration sequence

1. #696 establishes this naming standard and the compatibility baseline.
2. #695 defines the authoritative knowledge registry schema and seed registry.
3. #697 ports validation into automated checks.
4. #699 records accepted domains and import batches in the opening ledger.
5. #692 defines how validated entries become available to runtime systems.
6. #693 migrates legacy taxonomy content incrementally, preserving lineage.

## Non-goals

This standard does not:

- Decide the final reading-and-writing taxonomy placement.
- Migrate all legacy content.
- Make knowledge entries runtime-enabled.
- Replace evidence review, CARE governance, or access-control policy.
- Determine database, graph, vector-store, or API implementation details.
