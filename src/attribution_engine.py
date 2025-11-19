from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple
from datetime import datetime, timedelta
import uuid

@dataclass
class Node:
    node_id: str
    node_type: str
    tenant_id: str
    source_system: str
    event_code: Optional[str] = None
    timestamp: Optional[datetime] = None
    metadata: dict = field(default_factory=dict)

@dataclass
class Edge:
    edge_id: str
    from_node_id: str
    to_node_id: str
    edge_type: str
    causal_weight: float
    confidence_score: float
    metadata: dict = field(default_factory=dict)

CANONICAL_EVENT_MAP = {
    "TMS_TENDER_REJECT": "TENDER_REJECTED",
    "TMS_RE_BID": "RE_BID_REQUESTED",
    "TMS_CARRIER_ASSIGNED": "CARRIER_ASSIGNED",
    "WEATHER_STORM": "WEATHER_DELAY",
    "IN_TRANSIT_DELAY": "IN_TRANSIT_DELAY",
    "CARRIER_STATUS_LATE": "IN_TRANSIT_DELAY",
    "DETENTION_START": "DETENTION_START",
    "DETENTION_END": "DETENTION_END",
    "DELIVERED_LATE": "DELIVERED_LATE",
    "TMS_TENDER_OFFER": "TENDER_OFFER",
    "PICKUP_COMPLETE": "PICKUP_COMPLETE",
    "ARRIVAL_AT_DEST": "ARRIVAL_AT_DEST",
    "DELIVERED_POD": "DELIVERED_POD"
}

def normalize_raw_event(raw_event: dict) -> dict:
    src_code = raw_event.get("event_type") or raw_event.get("risk_type")
    event_code = CANONICAL_EVENT_MAP.get(src_code, "UNKNOWN")
    
    ts = raw_event.get("event_ts")
    if isinstance(ts, str):
        try:
            ts = datetime.fromisoformat(ts.replace("Z", "+00:00"))
        except ValueError:
            pass

    return {
        "node_id": raw_event.get("event_id"),
        "event_code": event_code,
        "event_timestamp": ts,
        "source_system": raw_event.get("source_system"),
        "related_shipment_id": raw_event.get("shipment_id"),
        "raw_payload": raw_event
    }

class CausalChainBuilder:
    def __init__(self):
        self.nodes: Dict[str, Node] = {}
        self.edges: List[Edge] = []
        self.shipment_events: Dict[str, List[Node]] = {}

    def _create_node(self, normalized_event: dict) -> Node:
        node = Node(
            node_id=normalized_event["node_id"],
            node_type="EVENT",
            tenant_id="TENANT-01",
            source_system=normalized_event["source_system"],
            event_code=normalized_event["event_code"],
            timestamp=normalized_event["event_timestamp"],
            metadata=normalized_event["raw_payload"]
        )
        self.nodes[node.node_id] = node
        
        shp_id = normalized_event.get("related_shipment_id")
        if shp_id:
            if shp_id not in self.shipment_events:
                self.shipment_events[shp_id] = []
            self.shipment_events[shp_id].append(node)
            self.shipment_events[shp_id].sort(key=lambda x: x.timestamp if x.timestamp else datetime.min)
            
        return node

    def _create_edge(self, from_node: Node, to_node: Node, edge_type: str, weight: float, confidence: float):
        edge = Edge(
            edge_id=f"EDGE-{uuid.uuid4().hex[:8]}",
            from_node_id=from_node.node_id,
            to_node_id=to_node.node_id,
            edge_type=edge_type,
            causal_weight=weight,
            confidence_score=confidence
        )
        self.edges.append(edge)
        return edge

    def process_event(self, raw_event: dict) -> Node:
        norm = normalize_raw_event(raw_event)
        node = self._create_node(norm)
        self._apply_causal_heuristics(node)
        return node

    def _apply_causal_heuristics(self, current_node: Node):
        if not current_node.metadata.get("shipment_id"):
            return

        shp_id = current_node.metadata.get("shipment_id")
        timeline = self.shipment_events.get(shp_id, [])
        
        prior_events = [n for n in timeline if n.timestamp and current_node.timestamp and n.timestamp < current_node.timestamp]
        
        # Debug check (removed in prod)
        # print(f"Current: {current_node.event_code} at {current_node.timestamp}. Prior candidates: {len(prior_events)}")
        # print([e.event_code for e in prior_events])

        if current_node.event_code == "RE_BID_REQUESTED":
            cause = self._find_most_recent(prior_events, "TENDER_REJECTED")
            if cause:
                self._create_edge(cause, current_node, "CAUSES", 1.0, 1.0)

        elif current_node.event_code == "CARRIER_ASSIGNED":
            cause = self._find_most_recent(prior_events, "RE_BID_REQUESTED")
            if cause:
                 self._create_edge(cause, current_node, "CAUSES", 0.8, 0.9)

        elif current_node.event_code == "IN_TRANSIT_DELAY":
            cause = self._find_most_recent(prior_events, "WEATHER_DELAY")
            if cause:
                self._create_edge(cause, current_node, "CAUSES", 0.9, 0.95)

        elif current_node.event_code == "DETENTION_START":
             cause = self._find_most_recent(prior_events, "ARRIVAL_AT_DEST")
             if cause:
                 self._create_edge(cause, current_node, "PRECEDES", 1.0, 1.0)
             
             delay_cause = self._find_most_recent(prior_events, "IN_TRANSIT_DELAY")
             if delay_cause:
                  pass

    def _find_most_recent(self, events: List[Node], event_code: str) -> Optional[Node]:
        for event in reversed(events):
            if event.event_code == event_code:
                return event
        return None
