{{ config(materialized='table') }}

SELECT
    node_id,
    event_code,
    event_timestamp,
    event_severity,
    raw_event_payload,
    normalized_fields,
    related_shipment_id,
    related_order_id,
    related_location_id
FROM {{ ref('stg_events_combined') }}

