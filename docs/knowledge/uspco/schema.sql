
PRAGMA foreign_keys = ON;
CREATE TABLE super_capability (
  code TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  note TEXT NOT NULL
);
CREATE TABLE capability_part (
  id INTEGER PRIMARY KEY,
  capability_code TEXT NOT NULL REFERENCES super_capability(code),
  name TEXT NOT NULL
);
CREATE TABLE super_power_family (
  code TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  note TEXT NOT NULL
);
CREATE TABLE super_power (
  id INTEGER PRIMARY KEY,
  family_code TEXT NOT NULL REFERENCES super_power_family(code),
  name TEXT NOT NULL
);
CREATE TABLE capability_state (
  name TEXT PRIMARY KEY,
  position INTEGER NOT NULL UNIQUE
);
CREATE TABLE constraint_kind (
  name TEXT PRIMARY KEY
);
CREATE TABLE platform (
  name TEXT PRIMARY KEY,
  note TEXT NOT NULL
);
CREATE TABLE rule (
  name TEXT PRIMARY KEY,
  statement TEXT NOT NULL
);
