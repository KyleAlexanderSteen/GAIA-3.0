import sys, pathlib, unittest
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
import stub_detector as sd, dod_check as dc

STUB = 'pub async fn run() -> Result<()> {\n    println!("Starting");\n    // TODO: spawn\n    println!("\u2713 Runtime started.");\n    Ok(())\n}\n'
FIXED = 'pub async fn run() -> Result<()> {\n    // TODO: spawn\n    Err(super::not_implemented("start", "#1298"))\n}\n'
REAL = 'fn go() { do_work(); println!("done"); }\n'
ALL = tuple(dc.STAGES)


def make(stages=('SPECIFICATION',), ev=('SPECIFICATION: docs/example.md',), head='Refs #1', inline=False):
    boxes = '\n'.join(f"- [{'x' if s in stages else ' '}] {s}: x" for s in dc.STAGES)
    bullets = '\n'.join(f'- {e}' for e in ev)
    if inline:
        return f'{head}\n\n## Stage reached\n{boxes}\n\nEvidence:\n{bullets}\n\nAn issue is only closed as complete when OPERATIONAL evidence is linked.\n\n## Layer\n- [ ] docs\n'
    return f'{head}\n\n## Stage reached\n{boxes}\n\n## Evidence\n{bullets}\n'


class Stub(unittest.TestCase):
    def test_flags_stub(self): self.assertEqual(sd.find_stubs(STUB), [('run', 1)])
    def test_ignores_loud_failure(self): self.assertEqual(sd.find_stubs(FIXED), [])
    def test_ignores_real_work_without_todo(self): self.assertEqual(sd.find_stubs(REAL), [])


class Dod(unittest.TestCase):
    def test_good_evidence_section(self): self.assertEqual(dc.check(make()), [])
    def test_good_template_style_evidence(self): self.assertEqual(dc.check(make(inline=True)), [])
    def test_missing_section(self): self.assertTrue(dc.check('## Summary\nhi'))
    def test_none_checked(self): self.assertTrue(dc.check('## Stage reached\n- [ ] TEST\n'))
    def test_closing_keywords_need_operational(self):
        for word in ('Close', 'Closes', 'Closed', 'Fix', 'Fixes', 'Fixed', 'Resolve', 'Resolves', 'Resolved'):
            with self.subTest(word=word):
                self.assertTrue(dc.check(make(head=f'{word} #1')))
    def test_lowercase_and_mid_line_keywords(self):
        self.assertTrue(dc.check(make(head='fixes #1')))
        self.assertTrue(dc.check(make(head='This fixes #1')))
    def test_operational_can_close(self):
        self.assertEqual(dc.check(make(ALL, ev=('OPERATIONAL: runs/1.json',), head='Closes #1')), [])
    def test_high_stage_requires_lower_stages(self):
        probs = dc.check(make(('SPECIFICATION', 'TEST'), ev=('TEST: tests/t.py',)))
        self.assertTrue(any('contiguous' in p for p in probs))
    def test_operational_alone_fails(self):
        probs = dc.check(make(('OPERATIONAL',), ev=('OPERATIONAL: runs/1.json',)))
        self.assertTrue(any('contiguous' in p for p in probs))
    def test_highest_stage_needs_its_own_evidence(self):
        probs = dc.check(make(('SPECIFICATION', 'IMPLEMENTATION')))
        self.assertTrue(any('IMPLEMENTATION' in p for p in probs))
    def test_no_evidence_fails(self):
        self.assertTrue(dc.check(make(ev=())))
        self.assertTrue(dc.check(make(ev=(), inline=True)))


if __name__ == '__main__':
    unittest.main()
