# RAG 15 failure modes — listed (#906)

Baseline: `docs/research/rag-baseline.md`. Code already on main: hybrid, tier2, cite, grounding.

| FM | Status |
| --- | --- |
| 1 chunks | partial — chunking.md |
| 2 embeddings | partial — MockEmbedder |
| 3 irrelevant | partial — hybrid + min score |
| 4 metadata | partial — metadata-standard.md |
| 5 freshness | listed — freshness.md; no TTL runtime |
| 6 query | partial — tier2 expansion |
| 7 conflict | partial — tier2 flag |
| 8 overload | partial — char budget + MMR |
| 9 citations | partial — retrieve_and_cite |
| 10 grounding | partial — enforce_grounding |
| 11 confidence | partial — NeedVerify |
| 12 structure | absent |
| 13 preprocess | partial — JSONL store |
| 14 evolution | refused trainer |
| 15 multi-hop agent | absent |

No new retrieve API in this PR.
