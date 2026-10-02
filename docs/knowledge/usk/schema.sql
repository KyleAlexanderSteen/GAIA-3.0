-- Universal Subjects of Knowledge. Listed ontology. Not a live ingest.
-- SI consciousness is not an axiom.

PRAGMA foreign_keys = ON;

CREATE TABLE domain (
  code TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  role TEXT NOT NULL
);

CREATE TABLE subject (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL UNIQUE
);

CREATE TABLE subject_domain (
  subject_id INTEGER NOT NULL REFERENCES subject(id),
  domain_code TEXT NOT NULL REFERENCES domain(code),
  PRIMARY KEY (subject_id, domain_code)
);

CREATE TABLE knowledge_mode (
  name TEXT PRIMARY KEY,
  question TEXT NOT NULL
);

CREATE TABLE relationship_type (
  name TEXT PRIMARY KEY
);

CREATE TABLE evidence_kind (
  name TEXT PRIMARY KEY
);

CREATE TABLE intelligence (
  name TEXT PRIMARY KEY,
  relation_note TEXT NOT NULL
);

CREATE TABLE claim (
  id INTEGER PRIMARY KEY,
  subject_id INTEGER NOT NULL REFERENCES subject(id),
  statement TEXT NOT NULL,
  certainty TEXT NOT NULL CHECK (certainty IN ('unresolved', 'supported', 'refused')),
  UNIQUE (subject_id, statement)
);
