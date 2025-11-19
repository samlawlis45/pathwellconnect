{{ config(materialized='view') }}

-- Placeholder logic for SKU Margin Impact
-- In reality, this would join Order Lines to Shipments to Events to Charges
-- and allocate costs based on weight/value/quantity.

SELECT
    o.node_id as order_node_id,
    -- o.sku_id, -- Assuming SKU details are in order lines, which we don't have in core nodes yet. 
    -- For MVP, we might assume order level or mocked SKU level.
    s.shipment_number,
    sum(c.amount) as total_additional_costs,
    -- Mocked margin calculation
    0.28 as pre_event_margin,
    0.24 as post_event_margin,
    -0.04 as margin_erosion
FROM {{ ref('nodes_shipments') }} s
JOIN {{ ref('nodes_events') }} e ON e.related_shipment_id = s.node_id
JOIN {{ ref('ledger_edges') }} le ON le.from_node_id = e.node_id AND le.edge_type = 'RESULTS_IN'
JOIN {{ ref('nodes_charges') }} c ON c.node_id = le.to_node_id
LEFT JOIN {{ ref('ledger_edges') }} le_ord ON le_ord.from_node_id = s.node_id AND le_ord.edge_type = 'BELONGS_TO'
LEFT JOIN {{ ref('ledger_nodes') }} o ON o.node_id = le_ord.to_node_id AND o.node_type = 'ORDER'
GROUP BY 1, 2

