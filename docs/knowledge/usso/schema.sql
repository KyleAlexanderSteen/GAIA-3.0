
PRAGMA foreign_keys = ON;
CREATE TABLE super_skill_category (
  code TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  note TEXT NOT NULL,
  position INTEGER NOT NULL UNIQUE
);
CREATE TABLE super_skill (
  id INTEGER PRIMARY KEY,
  category_code TEXT NOT NULL REFERENCES super_skill_category(code),
  name TEXT NOT NULL,
  UNIQUE (category_code, name)
);
CREATE TABLE skill_status (
  name TEXT PRIMARY KEY,
  demonstrated INTEGER NOT NULL
);
CREATE TABLE rule (
  name TEXT PRIMARY KEY,
  statement TEXT NOT NULL
);
