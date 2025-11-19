{{ config(materialized='view') }}

select node_id, 'SHIPMENT' as node_type, 'TENANT-01' as tenant_id, shipment_number as external_id, 'TMS' as source_system, current_timestamp() as created_at, current_timestamp() as updated_at, 'INTERNAL' as visibility_tier, cast(null as string) as role_mask, cast(null as string) as auth_lineage, cast(null as string) as metadata, true as is_active from {{ ref('stg_tms_shipments') }}
union all
select node_id, 'EVENT' as node_type, 'TENANT-01' as tenant_id, event_code as external_id, 'MIXED' as source_system, current_timestamp() as created_at, current_timestamp() as updated_at, 'INTERNAL' as visibility_tier, cast(null as string) as role_mask, cast(null as string) as auth_lineage, cast(null as string) as metadata, true as is_active from {{ ref('stg_events_combined') }}
union all
select node_id, 'CHARGE' as node_type, 'TENANT-01' as tenant_id, charge_code as external_id, 'AUDIT' as source_system, current_timestamp() as created_at, current_timestamp() as updated_at, 'INTERNAL' as visibility_tier, cast(null as string) as role_mask, cast(null as string) as auth_lineage, cast(null as string) as metadata, true as is_active from {{ ref('stg_audit_charges') }}
union all
select node_id, 'INVOICE' as node_type, 'TENANT-01' as tenant_id, invoice_number as external_id, 'AUDIT' as source_system, current_timestamp() as created_at, current_timestamp() as updated_at, 'INTERNAL' as visibility_tier, cast(null as string) as role_mask, cast(null as string) as auth_lineage, cast(null as string) as metadata, true as is_active from {{ ref('stg_audit_invoices') }}
union all
select node_id, 'PARTY' as node_type, 'TENANT-01' as tenant_id, party_name as external_id, 'MASTER' as source_system, current_timestamp() as created_at, current_timestamp() as updated_at, 'INTERNAL' as visibility_tier, cast(null as string) as role_mask, cast(null as string) as auth_lineage, cast(null as string) as metadata, true as is_active from {{ ref('stg_parties_combined') }}
union all
select node_id, 'LOCATION' as node_type, 'TENANT-01' as tenant_id, name as external_id, 'MASTER' as source_system, current_timestamp() as created_at, current_timestamp() as updated_at, 'INTERNAL' as visibility_tier, cast(null as string) as role_mask, cast(null as string) as auth_lineage, cast(null as string) as metadata, true as is_active from {{ ref('stg_locations_combined') }}
union all
select node_id, 'RISK_EVENT' as node_type, 'TENANT-01' as tenant_id, risk_type as external_id, 'RISK' as source_system, current_timestamp() as created_at, current_timestamp() as updated_at, 'INTERNAL' as visibility_tier, cast(null as string) as role_mask, cast(null as string) as auth_lineage, cast(null as string) as metadata, true as is_active from {{ ref('stg_risk_events') }}
