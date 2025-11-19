{{ config(materialized='view') }}

SELECT
    location_id as node_id,
    location_id as location_code,
    name,
    cast(null as string) as address_line1,
    city,
    state as state_region,
    cast(null as string) as postal_code,
    country as country_code,
    lat as latitude,
    lon as longitude
FROM {{ ref('raw_locations') }}
