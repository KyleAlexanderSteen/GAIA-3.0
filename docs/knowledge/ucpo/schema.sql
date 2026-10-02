
PRAGMA foreign_keys = ON;
CREATE TABLE layer_kind (
  name TEXT PRIMARY KEY,
  meaning TEXT NOT NULL
);
CREATE TABLE capability_category (
  code TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  note TEXT NOT NULL,
  position INTEGER NOT NULL UNIQUE
);
CREATE TABLE capability (
  id INTEGER PRIMARY KEY,
  category_code TEXT NOT NULL REFERENCES capability_category(code),
  name TEXT NOT NULL
);
CREATE TABLE super_capability (
  code TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  note TEXT NOT NULL
);
CREATE TABLE power_family (
  code TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  note TEXT NOT NULL
);
CREATE TABLE power (
  id INTEGER PRIMARY KEY,
  family_code TEXT NOT NULL REFERENCES power_family(code),
  name TEXT NOT NULL
);
CREATE TABLE power_state (
  name TEXT PRIMARY KEY,
  position INTEGER NOT NULL UNIQUE
);
CREATE TABLE agent_class (
  name TEXT PRIMARY KEY,
  note TEXT NOT NULL
);
CREATE TABLE constraint_kind (
  name TEXT PRIMARY KEY,
  question TEXT NOT NULL
);
CREATE TABLE chain_step (
  position INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  direction TEXT NOT NULL
);
CREATE TABLE rule (
  name TEXT PRIMARY KEY,
  statement TEXT NOT NULL
);
