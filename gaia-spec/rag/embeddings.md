# EmbeddingModel — FM-2 trait contract and selection criteria

Issue: [#945](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/945)
Listed trait child: [#946](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/946) (`gaia-ukd`)
Wiring child: [#947](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/947) (`gaia-ingest` / `gaia-aikd`)
Parent: [#906](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/906)

This file is the listed contract. It does not download a model. It does not
stand up a vector database. Offline sovereignty still applies.

## Two surfaces (honest split)

GAIA currently ships two embedding surfaces. They are related but not identical.

| Surface | Crate | Role |
| --- | --- | --- |
| Listed design trait | `gaia-ukd::embed` | `EmbeddingModel` + `MockEmbedder` from #946 |
| Pipeline wiring trait | `gaia-ingest::embed` | `EmbeddingModel` + `PassthroughEmbedder` used by ingest/retrieval |

The pipeline trait is what `IngestPipeline` and `gaia_aikd::embed_query` call.
The listed `gaia-ukd` trait remains the design-gate stand-in. A future
`gaia-embed` adapter crate should implement both or collapse them.

## Offline adapters shipped in-tree (#999)

| Adapter | Dim | What it is |
| --- | --- | --- |
| `PassthroughEmbedder` | 1 | CI stub. Not retrieval-quality. |
| `HashingEmbedder` | 384 | Signed hashing trick + L2 norm. Lexical overlap only. `model_id = hashing-384-offline-v0`. |

Neither adapter downloads MiniLM. Neither is a production semantic model.

## Persistent citation index (#999)

`FileChunkStore` writes JSONL (`hex`, `text`, `model_id`, optional embedding).
Reload via `FileChunkStore::open`. This is not Qdrant.

## Pipeline trait contract (`gaia-ingest::embed`)

```text
EmbeddingModel: Debug + Send + Sync
  embed(&[&str]) -> Result<Vec<EmbeddingVector>, EmbedError>
  dim() -> Option<usize>
  model_id() -> &str
```

## Refuse

- No cloud embedding API in Tier 1
- No claim that hashing or passthrough vectors are MiniLM-quality
- No sentient or planetary embedding space
