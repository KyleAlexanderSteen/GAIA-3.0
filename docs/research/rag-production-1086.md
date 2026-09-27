# RAG in production chart — #1086

| Chart box | GAIA |
| --- | --- |
| 1 Index parse/chunk/embed/store | listed / local fixtures; no Qdrant Cloud |
| 2 Query rewrite/retrieve/rerank/assemble/generate | retrieve_and_cite + hybrid listed; no live LLM |
| 3 Vector + metadata + freshness | catalog.json; runtime_enabled false |
| 4 Hybrid / filters / top-k / rerank | hybrid.rs + tier2; no cross-encoder |
| 5 Grounded prompt / citations / fallback | GroundingClaim + NeedVerify |
| 6 Postgres / Redis / S3 / vector | **absent / refused** live |
| 7 Latency recall quality hallucination cost | eval listed; no cost meter |
| 8 ACL / PII / prompt defense / cache / feedback | tool_auth + halt; no PII redaction crate |

Chart is a map, not a build ticket for S3.
