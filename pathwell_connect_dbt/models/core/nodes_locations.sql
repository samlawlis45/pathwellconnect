{{ config(materialized='table') }}

SELECT
    node_id,
    location_code,
    name,
    address_line1,
    city,
    state_region,
    postal_code,
    country_code,
    latitude,
    longitude
FROM {{ ref('stg_locations_combined') }}

