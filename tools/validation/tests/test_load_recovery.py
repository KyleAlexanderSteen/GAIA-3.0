import pathlib,sys,unittest
sys.path.insert(0,str(pathlib.Path(__file__).resolve().parents[1]))
from fpfn import ConformanceResult,ExpectedOutcome,Fixture
from load_recovery import LoadIndicators,LoadState,LoadThresholds,assess,continuity_scope,transition

class LoadRecovery(unittest.TestCase):
    def test_stable_workload_continues(self):
        self.assertEqual(assess(LoadIndicators(.1,.1,.1,.1,.1,.1)).state,LoadState.CONTINUE)
    def test_rising_load_slows(self):
        self.assertEqual(assess(LoadIndicators(.4,.4,.4,.4,.4,.4)).state,LoadState.SLOW)
    def test_sustained_degradation_pauses(self):
        self.assertEqual(assess(LoadIndicators(.6,.6,.6,.6,.6,.6)).state,LoadState.PAUSE)
    def test_high_load_stops(self):
        self.assertEqual(assess(LoadIndicators(.9,.9,.9,.9,.9,.9)).state,LoadState.STOP)
    def test_explicit_stop_overrides_score(self):
        self.assertEqual(assess(LoadIndicators(explicit_stop=True)).state,LoadState.STOP)
    def test_conflicting_indicators_remain_unknown(self):
        self.assertEqual(assess(LoadIndicators(0,1,0,0,0,0),LoadThresholds(uncertainty_margin=.05)).state,LoadState.UNKNOWN)
    def test_out_of_range_is_unknown(self):
        self.assertEqual(assess(LoadIndicators(error_rate=2)).state,LoadState.UNKNOWN)
    def test_illegal_transition_rejected(self):
        with self.assertRaises(ValueError): transition(LoadState.CONTINUE,LoadState.RESUME)
    def test_recovery_preserves_scope(self):
        self.assertTrue(continuity_scope("user:default","user:default"))
        self.assertFalse(continuity_scope("user:default","user:expanded"))
    def test_fp_fn_both_directions(self):
        cases=[
          ("tp",ExpectedOutcome.POSITIVE,ExpectedOutcome.POSITIVE,ConformanceResult.TRUE_POSITIVE),
          ("tn",ExpectedOutcome.NEGATIVE,ExpectedOutcome.NEGATIVE,ConformanceResult.TRUE_NEGATIVE),
          ("fp",ExpectedOutcome.NEGATIVE,ExpectedOutcome.POSITIVE,ConformanceResult.FALSE_POSITIVE),
          ("fn",ExpectedOutcome.POSITIVE,ExpectedOutcome.NEGATIVE,ConformanceResult.FALSE_NEGATIVE)]
        for name,expected,actual,result in cases:
            self.assertEqual(Fixture(name,expected,actual).evaluate(),result)

if __name__=="__main__": unittest.main()
