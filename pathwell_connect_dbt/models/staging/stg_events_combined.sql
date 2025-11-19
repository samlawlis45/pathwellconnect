{{ config(materialized='view') }}

with tms as (
    select
        event_id,
        shipment_id,
        event_type,
        event_ts,
        description,
        source_system
    from {{ ref('raw_tms_events') }}
),
vis as (
    select
        event_id,
        shipment_id,
        event_type,
        event_ts,
        description,
        source_system
    from {{ ref('raw_visibility_events') }}
),
combined as (
    select * from tms
    union all
    select * from vis
)

SELECT
    event_id as node_id,
    event_type as event_code,
    cast(event_ts as timestamp) as event_timestamp,
    'INFO' as event_severity,
    description as raw_event_payload,
    description as normalized_fields,
    shipment_id as related_shipment_id,
    cast(null as string) as related_order_id,
    cast(null as string) as related_location_id
FROM combined
