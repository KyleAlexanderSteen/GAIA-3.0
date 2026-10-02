import sys, pathlib, unittest
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
import stub_detector as sd, dod_check as dc

STUB = 'pub async fn run() -> Result<()> {\n    println!("Starting");\n    // TODO: spawn\n    println!("✓ Runtime started.");\n    Ok(())\n}\n'
FIXED = 'pub async fn run() -> Result<()> {\n    // TODO: spawn\n    Err(super::not_implemented("start", "#1298"))\n}\n'
REAL = 'fn go() { do_work(); println!("done"); }\n'
BASE = '''Refs #1

## Stage reached
- [x] SPECIFICATION: x
- [ ] IMPLEMENTATION
- [ ] TEST
- [ ] INTEGRATION
- [ ] VERIFICATION
- [ ] OPERATIONAL

## Evidence
- SPECIFICATION: docs/example.md
'''


class Stub(unittest.TestCase):
    def test_flags_stub(self): self.assertEqual(sd.find_stubs(STUB), [('run', 1)])
    def test_ignores_loud_failure(self): self.assertEqual(sd.find_stubs(FIXED), [])
    def test_ignores_real_work_without_todo(self): self.assertEqual(sd.find_stubs(REAL), [])


class Dod(unittest.TestCase):
    def test_good_specification(self): self.assertEqual(dc.check(BASE), [])
    def test_missing_section(self): self.assertTrue(dc.check('## Summary\nhi'))
    def test_none_checked(self): self.assertTrue(dc.check('## Stage reached\n- [ ] TEST\n'))
    def test_closing_keywords_need_operational(self):
        for word in ('Close', 'Closes', 'Closed', 'Fix', 'Fixes', 'Fixed', 'Resolve', 'Resolves', 'Resolved'):
            with self.subTest(word=word):
                self.assertTrue(dc.check(BASE.replace('Refs #1', f'{word} #1')))
    def test_operational_can_close(self):
        body = BASE.replace('- [ ] IMPLEMENTATION', '- [x] IMPLEMENTATION').replace('- [ ] TEST', '- [x] TEST').replace('- [ ] INTEGRATION', '- [x] INTEGRATION').replace('- [ ] VERIFICATION', '- [x] VERIFICATION').replace('- [ ] OPERATIONAL', '- [x] OPERATIONAL').replace('- SPECIFICATION: docs/example.md', '- OPERATIONAL: receipts/run-1.json').replace('Refs #1', 'Closes #1')
        self.assertEqual(dc.check(body), [])
    def test_lowercase_closing_keyword_needs_operational(self): self.assertTrue(dc.check(BASE.replace('Refs #1', 'fixes #1')))
    def test_mid_line_closing_keyword_needs_operational(self): self.assertTrue(dc.check(BASE.replace('Refs #1', 'This fixes #1')))
    def test_high_stage_requires_lower_stages(self):
        body = BASE.replace('- [ ] TEST', '- [x] TEST').replace('- SPECIFICATION: docs/example.md', '- TEST: tests/test_feature.py')
        self.assertTrue(any('contiguous' in p for p in dc.check(body)))
    def test_operational_alone_fails(self):
        body = BASE.replace('- [x] SPECIFICATION', '- [ ] SPECIFICATION').replace('- [ ] OPERATIONAL', '- [x] OPERATIONAL').replace('- SPECIFICATION: docs/example.md', '- OPERATIONAL: receipts/run-1.json')
        self.assertTrue(any('contiguous' in p for p in dc.check(body)))
    def test_highest_stage_requires_its_evidence(self):
        body = BASE.replace('- [ ] IMPLEMENTATION', '- [x] IMPLEMENTATION')
        self.assertTrue(any('IMPLEMENTATION' in p for p in dc.check(body)))
    def test_missing_evidence_section_fails(self): self.assertTrue(any('Evidence' in p for p in dc.check(BASE.split('## Evidence')[0])))
    def test_evidence_for_wrong_stage_fails(self):
        body = BASE.replace('- [ ] IMPLEMENTATION', '- [x] IMPLEMENTATION')
        self.assertTrue(any('IMPLEMENTATION' in p for p in dc.check(body)))


if __name__ == '__main__':
    unittest.main()
