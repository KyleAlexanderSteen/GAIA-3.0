
PRAGMA foreign_keys = ON;
CREATE TABLE super_capability (
  code TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  note TEXT NOT NULL,
  position INTEGER NOT NULL UNIQUE
);
CREATE TABLE capability_part (
  id INTEGER PRIMARY KEY,
  capability_code TEXT NOT NULL REFERENCES super_capability(code),
  name TEXT NOT NULL
);
CREATE TABLE super_power (
  code TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  note TEXT NOT NULL,
  position INTEGER NOT NULL UNIQUE
);
CREATE TABLE power_part (
  id INTEGER PRIMARY KEY,
  power_code TEXT NOT NULL REFERENCES super_power(code),
  name TEXT NOT NULL
);
CREATE TABLE required_property (
  name TEXT PRIMARY KEY,
  question TEXT NOT NULL
);
CREATE TABLE rule (
  name TEXT PRIMARY KEY,
  statement TEXT NOT NULL
);
