---
license: cc-by-4.0
task_categories:
  - tabular-classification
language:
  - en
tags:
  - gaia-2.0
  - lithic
  - elements
pretty_name: GAIA 2.0 Elements
---

# GAIA 2.0 Elements

Canonical element records for GAIA 2.0. Source of truth is Supabase table `public.elements` in project `gaia-2-0` (`yylqoiqobydrdsnnulip`). GitHub proof: `PROOF-GAIA20-STACK-001` in `docs/academic/STACK.md`.

## Schema

| Column | Type | Notes |
|---|---|---|
| z | int 1–118 | atomic number, PK |
| symbol | string | |
| name | string | |
| data | JSON | GAIA layer alignment, notes |

## Seed

Oxygen (8), Silicon (14), Iron (26).

Publish this folder to `KyleAlexanderSteen/gaia-elements` on the Hub when dataset-create credentials are available.
