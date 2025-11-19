{{ config(materialized='view') }}

SELECT
    edge_id,
    from_node_id,
    to_node_id,
    edge_type,
    'TENANT-01' as tenant_id,
    current_timestamp() as created_at,
    current_timestamp() as updated_at,
    causal_weight,
    1.0 as confidence_score,
    'INTERNAL' as visibility_tier,
    cast(null as string) as role_mask,
    cast(null as string) as metadata
FROM {{ ref('raw_ledger_edges') }}
