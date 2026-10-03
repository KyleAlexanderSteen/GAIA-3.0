# Vector databases — listed (#862)

**Points at:** `gaia-memos`, `gaia-aikd`, `gaia-ukd`, `gaia-ingest`.

Named field: Pinecone, Qdrant, Weaviate, Milvus, Chroma, pgvector.

| Question | Listed answer |
| --- | --- |
| What runs in-repo | `FileChunkStore` JSONL + hashing embedder |
| Supabase pgvector | Repo has `supabase/` paths; not stood up here |
| Hybrid retrieve | `gaia-aikd` BM25-lite + cosine RRF |
| Qdrant / SFS | [#723](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/723) stays open |

## Refuse

- No Qdrant process. No Pinecone account. No `gaia-memory-spec` crate.
