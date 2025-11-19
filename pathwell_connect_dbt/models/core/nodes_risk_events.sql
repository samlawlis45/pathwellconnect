{{ config(materialized='table') }}

SELECT
    node_id,
    risk_type,
    severity,
    risk_source,
    event_timestamp,
    geo_context,
    description
FROM {{ ref('stg_risk_events') }}

