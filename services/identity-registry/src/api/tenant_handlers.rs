use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use uuid::Uuid;
use chrono::Utc;

use crate::api::models::*;
use crate::api::routes::AppState;
use crate::db::models::{Tenant, TenantType};

fn parse_tenant_type(s: &str) -> TenantType {
    match s.to_lowercase().as_str() {
        "platform" => TenantType::Platform,
        "parent" => TenantType::Parent,
        "child" => TenantType::Child,
        "instance" => TenantType::Instance,
        _ => TenantType::Child,
    }
}

fn tenant_type_to_string(t: TenantType) -> String {
    match t {
        TenantType::Platform => "platform".to_string(),
        TenantType::Parent => "parent".to_string(),
        TenantType::Child => "child".to_string(),
        TenantType::Instance => "instance".to_string(),
    }
}

pub async fn create_tenant(
    State(state): State<AppState>,
    Json(payload): Json<CreateTenantRequest>,
) -> Result<Json<CreateTenantResponse>, (StatusCode, Json<ErrorResponse>)> {
    let pool = &state.pool;

    // Check if tenant already exists
    let existing = sqlx::query!(
        "SELECT id FROM tenants WHERE tenant_id = $1",
        payload.tenant_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "database_error".to_string(),
                message: e.to_string(),
            }),
        )
    })?;

    if existing.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                error: "tenant_exists".to_string(),
                message: format!("Tenant {} already exists", payload.tenant_id),
            }),
        ));
    }

    // Resolve parent tenant if provided
    let parent_id = if let Some(ref parent_tid) = payload.parent_tenant_id {
        let parent = sqlx::query!("SELECT id FROM tenants WHERE tenant_id = $1", parent_tid)
            .fetch_optional(pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "database_error".to_string(),
                        message: e.to_string(),
                    }),
                )
            })?;

        parent.map(|p| p.id).ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "parent_not_found".to_string(),
                    message: format!("Parent tenant {} not found", parent_tid),
                }),
            )
        })?
    } else {
        // No parent - create as root tenant
        return create_root_tenant(State(state), payload).await;
    };

    let tenant_type = payload
        .tenant_type
        .as_ref()
        .map(|s| parse_tenant_type(s))
        .unwrap_or(TenantType::Child);

    let governance = payload
        .governance_config
        .unwrap_or_else(|| serde_json::json!({"policy_scope": "inherit"}));
    let visibility = payload
        .visibility_config
        .unwrap_or_else(|| serde_json::json!({"cross_tenant_visibility": "none"}));

    let id = Uuid::new_v4();
    let now = Utc::now().naive_utc();

    // Insert tenant - trigger will handle hierarchy_depth, hierarchy_path, root_tenant_id
    let tenant = sqlx::query_as!(
        Tenant,
        r#"
        INSERT INTO tenants (
            id, tenant_id, tenant_type, display_name, parent_tenant_id,
            governance_config, visibility_config, metadata, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING
            id, tenant_id, tenant_type as "tenant_type: TenantType", display_name,
            parent_tenant_id, root_tenant_id, hierarchy_depth, hierarchy_path,
            governance_config, visibility_config, metadata,
            created_at, updated_at, deactivated_at
        "#,
        id,
        payload.tenant_id,
        tenant_type as TenantType,
        payload.display_name,
        Some(parent_id),
        governance,
        visibility,
        payload.metadata,
        now,
        now
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "database_error".to_string(),
                message: e.to_string(),
            }),
        )
    })?;

    Ok(Json(CreateTenantResponse {
        id: tenant.id,
        tenant_id: tenant.tenant_id,
        tenant_type: tenant_type_to_string(tenant.tenant_type),
        hierarchy_depth: tenant.hierarchy_depth,
        hierarchy_path: tenant.hierarchy_path.unwrap_or_default(),
        created_at: Utc::now().to_rfc3339(),
    }))
}

async fn create_root_tenant(
    State(state): State<AppState>,
    payload: CreateTenantRequest,
) -> Result<Json<CreateTenantResponse>, (StatusCode, Json<ErrorResponse>)> {
    let pool = &state.pool;

    let tenant_type = payload
        .tenant_type
        .as_ref()
        .map(|s| parse_tenant_type(s))
        .unwrap_or(TenantType::Platform);

    let governance = payload
        .governance_config
        .unwrap_or_else(|| serde_json::json!({"policy_scope": "root"}));
    let visibility = payload
        .visibility_config
        .unwrap_or_else(|| serde_json::json!({"cross_tenant_visibility": "none"}));

    let id = Uuid::new_v4();
    let now = Utc::now().naive_utc();

    let tenant = sqlx::query_as!(
        Tenant,
        r#"
        INSERT INTO tenants (
            id, tenant_id, tenant_type, display_name, parent_tenant_id,
            governance_config, visibility_config, metadata, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, NULL, $5, $6, $7, $8, $9)
        RETURNING
            id, tenant_id, tenant_type as "tenant_type: TenantType", display_name,
            parent_tenant_id, root_tenant_id, hierarchy_depth, hierarchy_path,
            governance_config, visibility_config, metadata,
            created_at, updated_at, deactivated_at
        "#,
        id,
        payload.tenant_id,
        tenant_type as TenantType,
        payload.display_name,
        governance,
        visibility,
        payload.metadata,
        now,
        now
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "database_error".to_string(),
                message: e.to_string(),
            }),
        )
    })?;

    Ok(Json(CreateTenantResponse {
        id: tenant.id,
        tenant_id: tenant.tenant_id,
        tenant_type: tenant_type_to_string(tenant.tenant_type),
        hierarchy_depth: tenant.hierarchy_depth,
        hierarchy_path: tenant.hierarchy_path.unwrap_or_default(),
        created_at: Utc::now().to_rfc3339(),
    }))
}

pub async fn get_tenant(
    State(state): State<AppState>,
    Path(tenant_id): Path<String>,
) -> Result<Json<TenantResponse>, (StatusCode, Json<ErrorResponse>)> {
    let pool = &state.pool;

    let tenant = sqlx::query_as!(
        Tenant,
        r#"
        SELECT
            id, tenant_id, tenant_type as "tenant_type: TenantType", display_name,
            parent_tenant_id, root_tenant_id, hierarchy_depth, hierarchy_path,
            governance_config, visibility_config, metadata,
            created_at, updated_at, deactivated_at
        FROM tenants WHERE tenant_id = $1 AND deactivated_at IS NULL
        "#,
        tenant_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "database_error".to_string(),
                message: e.to_string(),
            }),
        )
    })?;

    let tenant = tenant.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "tenant_not_found".to_string(),
                message: format!("Tenant {} not found", tenant_id),
            }),
        )
    })?;

    Ok(Json(TenantResponse {
        id: tenant.id,
        tenant_id: tenant.tenant_id,
        tenant_type: tenant_type_to_string(tenant.tenant_type),
        display_name: tenant.display_name,
        parent_tenant_id: tenant.parent_tenant_id,
        root_tenant_id: tenant.root_tenant_id,
        hierarchy_depth: tenant.hierarchy_depth,
        hierarchy_path: tenant.hierarchy_path,
        governance_config: tenant.governance_config,
        visibility_config: tenant.visibility_config,
        metadata: tenant.metadata,
        created_at: tenant.created_at.and_utc().to_rfc3339(),
        updated_at: tenant.updated_at.and_utc().to_rfc3339(),
    }))
}

pub async fn get_tenant_hierarchy(
    State(state): State<AppState>,
    Path(tenant_id): Path<String>,
) -> Result<Json<TenantHierarchyResponse>, (StatusCode, Json<ErrorResponse>)> {
    let pool = &state.pool;

    // Get the tenant
    let tenant = sqlx::query_as!(
        Tenant,
        r#"
        SELECT
            id, tenant_id, tenant_type as "tenant_type: TenantType", display_name,
            parent_tenant_id, root_tenant_id, hierarchy_depth, hierarchy_path,
            governance_config, visibility_config, metadata,
            created_at, updated_at, deactivated_at
        FROM tenants WHERE tenant_id = $1 AND deactivated_at IS NULL
        "#,
        tenant_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "database_error".to_string(),
                message: e.to_string(),
            }),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "tenant_not_found".to_string(),
                message: format!("Tenant {} not found", tenant_id),
            }),
        )
    })?;

    // Get ancestors (walk up the hierarchy path)
    let ancestors: Vec<Tenant> = if let Some(ref path) = tenant.hierarchy_path {
        let path_vec: &Vec<String> = path;
        if path_vec.len() > 1 {
            let ancestor_ids: Vec<&str> = path_vec[..path_vec.len() - 1].iter().map(|s| s.as_str()).collect();
            sqlx::query_as!(
                Tenant,
                r#"
                SELECT
                    id, tenant_id, tenant_type as "tenant_type: TenantType", display_name,
                    parent_tenant_id, root_tenant_id, hierarchy_depth, hierarchy_path,
                    governance_config, visibility_config, metadata,
                    created_at, updated_at, deactivated_at
                FROM tenants WHERE tenant_id = ANY($1) AND deactivated_at IS NULL
                ORDER BY hierarchy_depth
                "#,
                &ancestor_ids
            )
            .fetch_all(pool)
            .await
            .unwrap_or_default()
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    // Get children
    let children = sqlx::query_as!(
        Tenant,
        r#"
        SELECT
            id, tenant_id, tenant_type as "tenant_type: TenantType", display_name,
            parent_tenant_id, root_tenant_id, hierarchy_depth, hierarchy_path,
            governance_config, visibility_config, metadata,
            created_at, updated_at, deactivated_at
        FROM tenants WHERE parent_tenant_id = $1 AND deactivated_at IS NULL
        ORDER BY tenant_id
        "#,
        tenant.id
    )
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    Ok(Json(TenantHierarchyResponse {
        tenant: TenantSummary {
            id: tenant.id,
            tenant_id: tenant.tenant_id,
            tenant_type: tenant_type_to_string(tenant.tenant_type),
            display_name: tenant.display_name,
            hierarchy_depth: tenant.hierarchy_depth,
        },
        ancestors: ancestors
            .into_iter()
            .map(|t| TenantSummary {
                id: t.id,
                tenant_id: t.tenant_id,
                tenant_type: tenant_type_to_string(t.tenant_type),
                display_name: t.display_name,
                hierarchy_depth: t.hierarchy_depth,
            })
            .collect(),
        children: children
            .into_iter()
            .map(|t| TenantSummary {
                id: t.id,
                tenant_id: t.tenant_id,
                tenant_type: tenant_type_to_string(t.tenant_type),
                display_name: t.display_name,
                hierarchy_depth: t.hierarchy_depth,
            })
            .collect(),
    }))
}

pub async fn update_tenant(
    State(state): State<AppState>,
    Path(tenant_id): Path<String>,
    Json(payload): Json<UpdateTenantRequest>,
) -> Result<Json<TenantResponse>, (StatusCode, Json<ErrorResponse>)> {
    let pool = &state.pool;
    let now = Utc::now().naive_utc();

    // Build dynamic update
    let tenant = sqlx::query_as!(
        Tenant,
        r#"
        UPDATE tenants SET
            display_name = COALESCE($2, display_name),
            governance_config = COALESCE($3, governance_config),
            visibility_config = COALESCE($4, visibility_config),
            metadata = COALESCE($5, metadata),
            updated_at = $6
        WHERE tenant_id = $1 AND deactivated_at IS NULL
        RETURNING
            id, tenant_id, tenant_type as "tenant_type: TenantType", display_name,
            parent_tenant_id, root_tenant_id, hierarchy_depth, hierarchy_path,
            governance_config, visibility_config, metadata,
            created_at, updated_at, deactivated_at
        "#,
        tenant_id,
        payload.display_name,
        payload.governance_config,
        payload.visibility_config,
        payload.metadata,
        now
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "database_error".to_string(),
                message: e.to_string(),
            }),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "tenant_not_found".to_string(),
                message: format!("Tenant {} not found", tenant_id),
            }),
        )
    })?;

    Ok(Json(TenantResponse {
        id: tenant.id,
        tenant_id: tenant.tenant_id,
        tenant_type: tenant_type_to_string(tenant.tenant_type),
        display_name: tenant.display_name,
        parent_tenant_id: tenant.parent_tenant_id,
        root_tenant_id: tenant.root_tenant_id,
        hierarchy_depth: tenant.hierarchy_depth,
        hierarchy_path: tenant.hierarchy_path,
        governance_config: tenant.governance_config,
        visibility_config: tenant.visibility_config,
        metadata: tenant.metadata,
        created_at: tenant.created_at.and_utc().to_rfc3339(),
        updated_at: tenant.updated_at.and_utc().to_rfc3339(),
    }))
}

pub async fn deactivate_tenant(
    State(state): State<AppState>,
    Path(tenant_id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let pool = &state.pool;
    let now = Utc::now().naive_utc();

    let result = sqlx::query!(
        "UPDATE tenants SET deactivated_at = $1, updated_at = $1 WHERE tenant_id = $2 AND deactivated_at IS NULL",
        now,
        tenant_id
    )
    .execute(pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "database_error".to_string(),
                message: e.to_string(),
            }),
        )
    })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "tenant_not_found".to_string(),
                message: format!("Tenant {} not found or already deactivated", tenant_id),
            }),
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}
