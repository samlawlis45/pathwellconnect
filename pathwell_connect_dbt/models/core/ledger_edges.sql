{{ config(materialized='table') }}

SELECT
    edge_id,
    from_node_id,
    to_node_id,
    edge_type,
    tenant_id,
    created_at,
    updated_at,
    causal_weight,
    confidence_score,
    visibility_tier,
    role_mask,
    metadata
FROM {{ ref('stg_edges_combined') }}

