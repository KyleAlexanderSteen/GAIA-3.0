# Receipt source is not a filter

Follows #1380. Refs #1360. Leaves #1360 open.

A line can say `source=cli` or `source=gateway`. Audit still prints every recent line. It does not select by writer. This note does not add that filter.
