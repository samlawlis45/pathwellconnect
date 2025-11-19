{{ config(materialized='view') }}

SELECT
    event_id as node_id,
    risk_type,
    severity,
    risk_source,
    cast(event_ts as timestamp) as event_timestamp,
    concat(geo_lat, ',', geo_lon) as geo_context,
    description
FROM {{ ref('raw_risk_events') }}
