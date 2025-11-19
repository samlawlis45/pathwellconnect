{{ config(materialized='view') }}

SELECT
    charge_id as node_id,
    charge_type,
    charge_type as charge_code,
    currency,
    amount,
    'CONTRACT-2023' as contract_rate_ref,
    invoice_id,
    cast(null as timestamp) as applied_at
FROM {{ ref('raw_audit_charges') }}
