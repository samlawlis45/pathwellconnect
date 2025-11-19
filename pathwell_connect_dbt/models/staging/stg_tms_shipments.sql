{{ config(materialized='view') }}

SELECT
    shipment_id as node_id,
    shipment_id as shipment_number,
    order_id,
    'TRUCKLOAD' as mode,
    origin_loc_id as origin_location_id,
    dest_loc_id as destination_location_id,
    cast(planned_pickup as timestamp) as planned_pickup_at,
    cast(null as timestamp) as actual_pickup_at,
    cast(planned_delivery as timestamp) as planned_delivery_at,
    cast(null as timestamp) as actual_delivery_at,
    carrier_id as carrier_party_id,
    cast(null as string) as broker_party_id,
    cast(null as string) as shipper_party_id,
    status,
    'STANDARD' as service_level
FROM {{ ref('raw_tms_shipments') }}
