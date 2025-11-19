{{ config(materialized='table') }}

SELECT
  pathwell_uuid        AS node_id,
  node_type,
  tenant_id,
  external_id,
  source_system,
  created_at,
  updated_at,
  visibility_tier,
  role_mask,
  auth_lineage,
  metadata,
  is_active
FROM {{ ref('nodes_union_staging') }}

