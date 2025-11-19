{{ config(materialized='table') }}

SELECT
    node_id,
    charge_type,
    charge_code,
    currency,
    amount,
    contract_rate_ref,
    invoice_id,
    applied_at
FROM {{ ref('stg_audit_charges') }}

