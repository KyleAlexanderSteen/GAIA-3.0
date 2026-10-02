# Universal Subjects of Knowledge

Listed ontology for GAIA 2.0. Not a live ingest. `runtime_enabled` is false. GAIA 3.0 was deleted; this database stays on GAIA 2.0.

The list you sent names 12 domains and then enumerates U0 through U12. That is 13 domains. All 13 are loaded.

Load:

```bash
python - << 'PY'
import sqlite3
con = sqlite3.connect("usk.sqlite3")
con.executescript(open("docs/knowledge/usk/schema.sql").read())
con.executescript(open("docs/knowledge/usk/seed.sql").read())
print(con.execute("select count(*) from domain").fetchone()[0])
print(con.execute("select count(*) from subject").fetchone()[0])
PY
```

Checked load: 13 domains, 273 subjects, 293 domain links, 16 knowledge modes.

A subject may sit in more than one domain. Neuroscience is linked to life, mind, and consciousness. Consciousness studies has one claim: unresolved, and SI consciousness is not an axiom.

DDC, LCC, and UNESCO are not mapped in this cut. The slot is the subject table, not a fake crosswalk.

No harvest. No trainer.
