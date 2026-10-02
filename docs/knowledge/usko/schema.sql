
PRAGMA foreign_keys = ON;
CREATE TABLE layer (
  code TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  question TEXT NOT NULL,
  position INTEGER NOT NULL UNIQUE
);
CREATE TABLE layer_item (
  id INTEGER PRIMARY KEY,
  layer_code TEXT NOT NULL REFERENCES layer(code),
  name TEXT NOT NULL,
  UNIQUE (layer_code, name)
);
CREATE TABLE perspective (
  name TEXT PRIMARY KEY,
  note TEXT NOT NULL
);
CREATE TABLE unknown_kind (
  name TEXT PRIMARY KEY,
  known_unknown INTEGER NOT NULL CHECK (known_unknown IN (0, 1))
);
CREATE TABLE claim_status (
  name TEXT PRIMARY KEY
);
CREATE TABLE distinction (
  name TEXT PRIMARY KEY,
  note TEXT NOT NULL
);
CREATE TABLE causal_grade (
  name TEXT PRIMARY KEY
);
CREATE TABLE super_skill (
  name TEXT PRIMARY KEY,
  definition TEXT NOT NULL
);
CREATE TABLE learning_step (
  position INTEGER PRIMARY KEY,
  name TEXT NOT NULL
);
