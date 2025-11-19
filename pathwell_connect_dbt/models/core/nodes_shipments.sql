{{ config(materialized='table') }}

-- In a real scenario, this would select from staging models.
-- For MVP structure, we define the schema.

SELECT
    node_id,
    shipment_number,
    order_id,
    mode,
    origin_location_id,
    destination_location_id,
    planned_pickup_at,
    actual_pickup_at,
    planned_delivery_at,
    actual_delivery_at,
    carrier_party_id,
    broker_party_id,
    shipper_party_id,
    status,
    service_level
FROM {{ ref('stg_tms_shipments') }}

