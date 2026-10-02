
PRAGMA foreign_keys = ON;
CREATE TABLE skill_category (
  code TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  note TEXT NOT NULL,
  position INTEGER NOT NULL UNIQUE
);
CREATE TABLE skill (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL UNIQUE
);
CREATE TABLE skill_category_link (
  skill_id INTEGER NOT NULL REFERENCES skill(id),
  category_code TEXT NOT NULL REFERENCES skill_category(code),
  PRIMARY KEY (skill_id, category_code)
);
CREATE TABLE agent_scope (
  category_code TEXT NOT NULL REFERENCES skill_category(code),
  agent TEXT NOT NULL CHECK (agent IN ('human','ai','si','shared')),
  note TEXT NOT NULL,
  PRIMARY KEY (category_code, agent)
);
CREATE TABLE maturity_level (
  level INTEGER PRIMARY KEY,
  name TEXT NOT NULL
);
CREATE TABLE distinction (
  name TEXT PRIMARY KEY,
  note TEXT NOT NULL
);
CREATE TABLE learning_loop (
  position INTEGER PRIMARY KEY,
  name TEXT NOT NULL
);
CREATE TABLE evaluation_dimension (
  name TEXT PRIMARY KEY
);
