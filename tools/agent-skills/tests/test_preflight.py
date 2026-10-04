import pathlib, sys, tempfile, unittest
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
import preflight as pf

def body(stage="IMPLEMENTATION", evidence="tools/agent-skills/preflight.py", head="Refs #1700"):
    stages = ["SPECIFICATION", "IMPLEMENTATION", "TEST", "INTEGRATION", "VERIFICATION", "OPERATIONAL"]
    boxes = "\n".join(f"- [{'x' if stages.index(s) <= stages.index(stage) else ' '}] {s}: x" for s in stages)
    return f"{head}\n\n## Stage reached\n{boxes}\n\n## Evidence\n- {stage}: {evidence}\n"

class Preflight(unittest.TestCase):
    def test_valid_repository_path(self):
        with tempfile.TemporaryDirectory() as root:
            p = pathlib.Path(root) / 'tools' / 'agent-skills' / 'preflight.py'
            p.parent.mkdir(parents=True); p.write_text('x', encoding='utf-8')
            self.assertEqual(pf.check(body(), pathlib.Path(root)), [])

    def test_missing_repository_path_is_rejected(self):
        with tempfile.TemporaryDirectory() as root:
            problems = pf.check(body(evidence='tools/missing.py'), pathlib.Path(root))
            self.assertTrue(any('does not exist' in p for p in problems))

    def test_absolute_path_is_rejected(self):
        with tempfile.TemporaryDirectory() as root:
            problems = pf.check(body(evidence='/etc/passwd'), pathlib.Path(root))
            self.assertTrue(any('repository-relative' in p for p in problems))

    def test_url_is_not_treated_as_repository_path(self):
        with tempfile.TemporaryDirectory() as root:
            self.assertEqual(pf.check(body(evidence='https://github.com/KyleAlexanderSteen/GAIA-3.0/issues/1700'), pathlib.Path(root)), [])

    def test_test_name_is_not_treated_as_repository_path(self):
        with tempfile.TemporaryDirectory() as root:
            self.assertEqual(pf.check(body(evidence='test_good_evidence_section'), pathlib.Path(root)), [])

    def test_previous_evidence_shape_regression_fails(self):
        malformed = "Refs #1700\n\n## Stage reached\n- [x] SPECIFICATION: x\n- [x] IMPLEMENTATION: tools/agent-skills/preflight.py\n"
        problems = pf.check(malformed)
        self.assertTrue(any('missing evidence bullet' in p for p in problems))

    def test_unknown_is_not_upgraded_to_verified(self):
        b = body(evidence='UNKNOWN: insufficient evidence')
        self.assertEqual(pf.check(b), [])
        self.assertIn('UNKNOWN', b)
        self.assertNotIn('VERIFIED: UNKNOWN', b)

if __name__ == '__main__': unittest.main()
