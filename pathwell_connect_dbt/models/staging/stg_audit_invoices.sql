{{ config(materialized='view') }}

SELECT
    invoice_id as node_id,
    invoice_id as invoice_number,
    cast(invoice_date as timestamp) as invoice_date,
    total_amount,
    currency,
    carrier_id as carrier_party_id,
    cast(null as string) as shipper_party_id,
    status
FROM {{ ref('raw_audit_invoices') }}
