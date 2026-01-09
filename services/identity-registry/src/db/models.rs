use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDateTime};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Enterprise {
    pub id: Uuid,
    pub enterprise_id: String,
    pub public_key: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, FromRow)]
pub struct Developer {
    pub id: Uuid,
    pub developer_id: String,
    pub enterprise_id: Option<Uuid>,
    pub public_key: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, FromRow)]
pub struct Agent {
    pub id: Uuid,
    pub agent_id: String,
    pub developer_id: Uuid,
    pub enterprise_id: Option<Uuid>,
    pub public_key: String,
    pub certificate_chain: String,
    pub created_at: NaiveDateTime,
    pub revoked_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

