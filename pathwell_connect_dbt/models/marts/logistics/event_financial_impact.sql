{{ config(materialized='view') }}

SELECT
    e.node_id         AS event_node_id,
    e.event_code,
    e.event_timestamp,
    s.shipment_number,
    c.node_id         AS charge_node_id,
    c.charge_type,
    c.amount,
    inv.invoice_number,
    lp.party_name     AS liable_party_name,
    lp.party_type     AS liable_party_type
FROM {{ ref('nodes_events') }} e
JOIN {{ ref('ledger_edges') }} le_event_charge
  ON le_event_charge.from_node_id = e.node_id
 AND le_event_charge.edge_type = 'RESULTS_IN'
JOIN {{ ref('nodes_charges') }} c
  ON c.node_id = le_event_charge.to_node_id
LEFT JOIN {{ ref('ledger_edges') }} le_charge_invoice
  ON le_charge_invoice.from_node_id = c.node_id
 AND le_charge_invoice.edge_type = 'LINKED_INVOICE'
LEFT JOIN {{ ref('nodes_invoices') }} inv
  ON inv.node_id = le_charge_invoice.to_node_id
LEFT JOIN {{ ref('ledger_edges') }} le_event_shipment
  ON le_event_shipment.from_node_id = e.node_id
 AND le_event_shipment.edge_type = 'BELONGS_TO'
LEFT JOIN {{ ref('nodes_shipments') }} s
  ON s.node_id = le_event_shipment.to_node_id
LEFT JOIN {{ ref('ledger_edges') }} le_charge_responsible
  ON le_charge_responsible.from_node_id = c.node_id
 AND le_charge_responsible.edge_type = 'RESPONSIBLE_PARTY'
LEFT JOIN {{ ref('nodes_parties') }} lp
  ON lp.node_id = le_charge_responsible.to_node_id

