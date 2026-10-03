# Three-tier knowledge taxonomy — migration

**Status:** listed migration rules  
**Issue:** #693  
**Source plane:** `KyleAlexanderSteen/GAIA` `docs/knowledge/the-subjects-of-knowledge-for-humans`  
**Counts in ancestor:** basic 40, intermediate 71, mastery 67, total 178

## Decision

Keep three **learning stages**. Do not keep three **identity prefixes**.

| Ancestor tier folder prefix | GAIA 2.0 stage | Role |
|---|---|---|
| `basic-*` | `foundation` | literacy and civic floor |
| `intermediate-*` | `intermediate` | disciplinary depth |
| `mastery-*` | `mastery` | frontier / research |

`advanced` in an ancestor slug is **not** a fourth stage. Per #696 it is either a stage wording to drop or part of a discipline name under review.

## Identity vs placement

```text
wrong:  gaia.knowledge.basic.biology
right:  gaia.knowledge.life-sciences.biology
        learning_placements.stage = foundation | intermediate | mastery
```

One enduring domain MAY have several placements. `basic-biology` and `intermediate-advanced-biology` are aliases of the same biology domain until a content review splits them.

## What this PR does not do

- Does not copy 178 subject folders into GAIA 2.0.
- Does not claim the ancestor 178-count as this registry's count.
- Does not mount UKD.

Content moves one named subject per later PR.

## Seed coverage

`catalog.json` seeds:

- renamed foundation domains from #696 examples
- three missing foundation domains from #698
- reading-and-writing as decided in #700
- a short mastery frontier sample for #694

Full ancestor subject lists remain authoritative only in the ancestor `catalog.json`.

## Acceptance for #693

- [x] Three stages retained as metadata.
- [x] Tier prefixes banned from canonical ids.
- [x] Migration is incremental.
- [x] No bulk folder import.
