
PRAGMA foreign_keys = ON;
CREATE TABLE magic_layer (
  code TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  note TEXT NOT NULL,
  position INTEGER NOT NULL UNIQUE
);
CREATE TABLE magic_item (
  id INTEGER PRIMARY KEY,
  layer_code TEXT NOT NULL REFERENCES magic_layer(code),
  name TEXT NOT NULL
);
CREATE TABLE source_status (
  name TEXT PRIMARY KEY
);
CREATE TABLE evidence_class (
  name TEXT PRIMARY KEY,
  meaning TEXT NOT NULL,
  position INTEGER NOT NULL UNIQUE
);
CREATE TABLE hybrid (
  name TEXT PRIMARY KEY,
  note TEXT NOT NULL
);
CREATE TABLE rule (
  name TEXT PRIMARY KEY,
  statement TEXT NOT NULL
);
