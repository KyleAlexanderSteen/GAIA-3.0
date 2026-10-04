import pathlib,sys,unittest
sys.path.insert(0,str(pathlib.Path(__file__).resolve().parents[1]))
from energy_states import AuthorizationSnapshot,EnergyState,ResourceMetrics,WakeEvent,record_transition,transition,validate_recovery,validate_wake

class EnergyStates(unittest.TestCase):
    def test_canonical_sleep_wake_path(self):
        state=EnergyState.FULL
        for requested in (EnergyState.ACTIVE,EnergyState.READY,EnergyState.LOW_POWER,EnergyState.SLEEP,EnergyState.WAKE_CONDITION,EnergyState.WAKE,EnergyState.ACTIVE):
            state=transition(state,requested)
        self.assertEqual(state,EnergyState.ACTIVE)
    def test_illegal_transition_rejected(self):
        with self.assertRaises(ValueError): transition(EnergyState.SLEEP,EnergyState.ACTIVE)
    def test_wake_requires_authorization_and_evidence(self):
        auth=AuthorizationSnapshot("runtime:default","v1")
        self.assertTrue(validate_wake(WakeEvent("user",True,"evt-1","p1"),auth))
        self.assertFalse(validate_wake(WakeEvent("user",False,"evt-1","p1"),auth))
        self.assertFalse(validate_wake(WakeEvent("user",True,"","p1"),auth))
    def test_recovery_does_not_broaden_authorization(self):
        auth=AuthorizationSnapshot("runtime:default","v1")
        expanded=AuthorizationSnapshot("runtime:expanded","v2")
        self.assertTrue(validate_recovery(auth,auth,True))
        self.assertFalse(validate_recovery(auth,expanded,True))
    def test_transition_record_preserves_audit_fields(self):
        auth=AuthorizationSnapshot("runtime:default","v1")
        record=record_transition(EnergyState.SLEEP,EnergyState.WAKE_CONDITION,"authorized_event","evt-1",auth,True)
        self.assertEqual(record.previous,EnergyState.SLEEP)
        self.assertEqual(record.resulting,EnergyState.WAKE_CONDITION)
        self.assertEqual(record.authorization,auth)
        self.assertTrue(record.integrity_ok)
    def test_resource_metrics_are_explicit(self):
        metrics=ResourceMetrics(EnergyState.LOW_POWER,.2,50.0,.8,True,True)
        self.assertLess(metrics.energy_units,1.0)
        self.assertGreater(metrics.responsiveness,0.0)

if __name__=="__main__": unittest.main()
