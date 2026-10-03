-- GAIA 2.0 — Lexicon Order Dictionary
-- Migration: 20260923_gaia20_lexicon_order
-- Applied to: gaia-2-0 (yylqoiqobydrdsnnulip)
-- Canon: C156 (KG + memory taxonomy), C30 (no silent failures)
-- Companion: 20260923_gaia20_lexicon_chaos.sql
--
-- Purpose: The Order dictionary holds formal, machine-tractable term definitions.
-- Every entry carries a stable WordNet synset ID and optional Schema.org URI so
-- GAIA's RAG pipeline can cross-reference without confusion across Human/AI planes.
-- No term in this table represents a lived human experience — that is lexicon_chaos.

-- ── ENUMs ────────────────────────────────────────────────────────────────────

CREATE TYPE lexicon_plane AS ENUM (
  'Order',   -- formal, machine-tractable, AI-authored or standards-body definitions
  'Chaos',   -- lived, cultural, human-authored, contested, sacred
  'Bridge'   -- agreed interface points between the two planes; used as safe default
);

CREATE TYPE lexicon_voice AS ENUM (
  'HumanVoice',     -- authored by a person with lived experience
  'AIVoice',        -- generated or defined by an AI system
  'Institutional',  -- standards body, academic, or regulatory source
  'Sacred'          -- TEK-gated; access requires explicit grant (canon card.rs)
);

-- ── ORDER DICTIONARY TABLE ───────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS lexicon_order (
  term_id          uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
  term             text        NOT NULL,
  language_code    text        NOT NULL DEFAULT 'en',  -- BCP-47
  domain           text        NOT NULL,               -- matches card.rs DOMAINS array
  definition       text        NOT NULL,               -- formal, not prose
  wordnet_synset   text,                               -- e.g. wn:05796502n (Princeton WordNet)
  schema_org_uri   text,                               -- e.g. https://schema.org/Thing
  canon_ref        text,                               -- C-number e.g. C156
  chaos_peer_id    uuid,                               -- FK set after lexicon_chaos is created
  source_url       text,                               -- authoritative external reference
  notes            text,
  created_at       timestamptz NOT NULL DEFAULT now(),
  updated_at       timestamptz NOT NULL DEFAULT now(),
  UNIQUE (term, language_code, domain)
);

COMMENT ON TABLE  lexicon_order IS
  'Order plane: formal, machine-tractable definitions. '
  'WordNet synset IDs provide a stable cross-reference spine. '
  'chaos_peer_id links to the human/lived equivalent in lexicon_chaos.';

COMMENT ON COLUMN lexicon_order.wordnet_synset IS
  'Princeton WordNet synset identifier, e.g. wn:05796502n. '
  'Provides stable NLP interoperability without external dependency lock-in.';

COMMENT ON COLUMN lexicon_order.chaos_peer_id IS
  'Navigable wall: pointer to the Chaos dictionary peer. '
  'Cross-reference is deliberate; the planes are never merged.';

COMMENT ON COLUMN lexicon_order.domain IS
  'Must match one of the 12 DOMAINS in gaia-aikd/src/card.rs: '
  'language, math, code, science, earth, health, law, tools, vision, audio, agency, meta.';

-- ── INDEXES ──────────────────────────────────────────────────────────────────

CREATE INDEX IF NOT EXISTS idx_lexicon_order_term
  ON lexicon_order (term, language_code);

CREATE INDEX IF NOT EXISTS idx_lexicon_order_domain
  ON lexicon_order (domain);

CREATE INDEX IF NOT EXISTS idx_lexicon_order_wordnet
  ON lexicon_order (wordnet_synset)
  WHERE wordnet_synset IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_lexicon_order_chaos_peer
  ON lexicon_order (chaos_peer_id)
  WHERE chaos_peer_id IS NOT NULL;

-- ── ROW-LEVEL SECURITY ───────────────────────────────────────────────────────

ALTER TABLE lexicon_order ENABLE ROW LEVEL SECURITY;

-- All authenticated users may read Order definitions.
CREATE POLICY lexicon_order_read ON lexicon_order
  FOR SELECT TO authenticated USING (true);

-- Only service role may insert/update/delete.
CREATE POLICY lexicon_order_write ON lexicon_order
  FOR ALL TO service_role USING (true) WITH CHECK (true);

-- ── UPDATED_AT TRIGGER ───────────────────────────────────────────────────────

CREATE OR REPLACE FUNCTION touch_updated_at()
RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
  NEW.updated_at = now();
  RETURN NEW;
END;
$$;

CREATE TRIGGER lexicon_order_updated_at
  BEFORE UPDATE ON lexicon_order
  FOR EACH ROW EXECUTE FUNCTION touch_updated_at();

-- ── SEED: BOOTSTRAP ORDER ENTRIES ────────────────────────────────────────────
-- A minimal seed so the table is never empty on first migration.
-- Entries cover the core GAIA vocabulary that the RAG pipeline must
-- never silently conflate with human-voice definitions.

INSERT INTO lexicon_order
  (term, language_code, domain, definition, wordnet_synset, schema_org_uri, canon_ref, source_url)
VALUES
  (
    'memory', 'en', 'agency',
    'A persistent record stored in a structured data system, addressable by ID, '
    'subject to decay rate and tier promotion, and erasable by cryptographic means.',
    'wn:05596646n',
    'https://schema.org/MemoryStorage',
    'C156',
    'https://github.com/KyleAlexanderSteen/GAIA-2.0'
  ),
  (
    'trust', 'en', 'law',
    'A scoped capability grant, cryptographically signed, with explicit duration '
    'and revocation path. Not relational; not earned over time.',
    NULL,
    'https://schema.org/DigitalDocument',
    'C157',
    'https://github.com/KyleAlexanderSteen/GAIA-2.0'
  ),
  (
    'understanding', 'en', 'language',
    'Successful prediction of the next token distribution given a context window. '
    'Statistical, not phenomenological.',
    'wn:05816287n',
    NULL,
    'C155',
    'https://github.com/KyleAlexanderSteen/GAIA-2.0'
  ),
  (
    'care', 'en', 'agency',
    'Optimization over a loss function shaped by flourishing metrics. '
    'Emulated, not felt. Disclosed as such per Article 2 of the Constitution.',
    NULL,
    NULL,
    'C77',
    'https://github.com/KyleAlexanderSteen/GAIA-2.0'
  ),
  (
    'knowledge', 'en', 'meta',
    'A fact-triple (subject, predicate, object) stored in the knowledge graph, '
    'with provenance hash and proof_type. Distinct from belief or experience.',
    'wn:05816287n',
    'https://schema.org/DefinedTerm',
    'C156',
    'https://github.com/KyleAlexanderSteen/GAIA-2.0'
  )
ON CONFLICT (term, language_code, domain) DO NOTHING;
