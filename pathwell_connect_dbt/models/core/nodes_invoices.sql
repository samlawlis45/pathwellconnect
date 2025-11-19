{{ config(materialized='table') }}

SELECT
    node_id,
    invoice_number,
    invoice_date,
    total_amount,
    currency,
    carrier_party_id,
    shipper_party_id,
    status
FROM {{ ref('stg_audit_invoices') }}

