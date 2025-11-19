{{ config(materialized='table') }}

SELECT
    node_id,
    party_type,
    party_name,
    external_party_id,
    country_code
FROM {{ ref('stg_parties_combined') }}

