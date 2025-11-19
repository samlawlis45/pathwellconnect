{{ config(materialized='view') }}

SELECT
    party_id as node_id,
    type as party_type,
    name as party_name,
    party_id as external_party_id,
    'USA' as country_code
FROM {{ ref('raw_parties') }}
