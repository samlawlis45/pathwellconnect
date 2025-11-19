import unittest
from datetime import datetime
from attribution_engine import CausalChainBuilder, normalize_raw_event

class TestAttributionEngine(unittest.TestCase):
    def setUp(self):
        self.builder = CausalChainBuilder()

    def test_tender_rejection_chain(self):
        evt1 = {
            "event_id": "E1",
            "event_type": "TMS_TENDER_OFFER",
            "event_ts": "2023-10-01T10:00:00",
            "shipment_id": "SHP-1",
            "source_system": "TMS"
        }
        self.builder.process_event(evt1)

        evt2 = {
            "event_id": "E2",
            "event_type": "TMS_TENDER_REJECT",
            "event_ts": "2023-10-01T12:00:00",
            "shipment_id": "SHP-1",
            "source_system": "TMS"
        }
        self.builder.process_event(evt2)

        evt3 = {
            "event_id": "E3",
            "event_type": "TMS_RE_BID",
            "event_ts": "2023-10-01T12:30:00",
            "shipment_id": "SHP-1",
            "source_system": "TMS"
        }
        self.builder.process_event(evt3)

        edges = [e for e in self.builder.edges if e.to_node_id == "E3"]
        self.assertEqual(len(edges), 1)
        self.assertEqual(edges[0].from_node_id, "E2")
        self.assertEqual(edges[0].edge_type, "CAUSES")

    def test_weather_delay_chain(self):
        evt_risk = {
            "event_id": "RISK-1",
            "risk_type": "WEATHER_STORM",
            "event_ts": "2023-10-02T14:00:00",
            "shipment_id": "SHP-1",
            "source_system": "RISK"
        }
        self.builder.process_event(evt_risk)

        evt_delay = {
            "event_id": "VIS-1",
            "event_type": "CARRIER_STATUS_LATE",
            "event_ts": "2023-10-02T15:00:00",
            "shipment_id": "SHP-1",
            "source_system": "ELD"
        }
        self.builder.process_event(evt_delay)
        
        edges = [e for e in self.builder.edges if e.to_node_id == "VIS-1"]
        self.assertTrue(any(e.from_node_id == "RISK-1" for e in edges))

if __name__ == '__main__':
    unittest.main()
