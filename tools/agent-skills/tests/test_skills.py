import sys, pathlib, unittest
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
import stub_detector as sd, dod_check as dc

STUB = 'pub async fn run() -> Result<()> {\n    println!("Starting");\n    // TODO: spawn\n    println!("\u2713 Runtime started.");\n    Ok(())\n}\n'
FIXED = 'pub async fn run() -> Result<()> {\n    // TODO: spawn\n    Err(super::not_implemented("start", "#1298"))\n}\n'
REAL = 'fn go() { do_work(); println!("done"); }\n'
GOOD = '## Stage reached\n- [x] SPECIFICATION: x\n- [ ] OPERATIONAL\n\nRefs #1\n'


class Stub(unittest.TestCase):
    def test_flags_stub(self): self.assertEqual(sd.find_stubs(STUB), [('run', 1)])
    def test_ignores_loud_failure(self): self.assertEqual(sd.find_stubs(FIXED), [])
    def test_ignores_real_work_without_todo(self): self.assertEqual(sd.find_stubs(REAL), [])


class Dod(unittest.TestCase):
    def test_good(self): self.assertEqual(dc.check(GOOD), [])
    def test_missing_section(self): self.assertTrue(dc.check('## Summary\nhi'))
    def test_none_checked(self): self.assertTrue(dc.check('## Stage reached\n- [ ] TEST\n'))
    def test_closes_needs_operational(self):
        self.assertTrue(dc.check(GOOD.replace('Refs', 'Closes')))
        self.assertEqual(dc.check(GOOD.replace('- [ ] OPERATIONAL', '- [x] OPERATIONAL').replace('Refs', 'Closes')), [])


if __name__ == '__main__':
    unittest.main()
