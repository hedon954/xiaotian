use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "system_logs")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,

    #[sea_orm(column_type = "String(Some(100))", not_null)]
    pub log_id: String,

    #[sea_orm(column_type = "String(Some(20))", not_null)]
    pub level: String, // "error", "warn", "info"

    #[sea_orm(column_type = "Text", not_null)]
    pub message: String,

    #[sea_orm(column_type = "String(Some(100))", not_null)]
    pub module: String,

    #[sea_orm(column_type = "Integer", nullable)]
    pub feed_id: Option<i32>,

    #[sea_orm(column_type = "Text", nullable)]
    pub error: Option<String>,

    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub timestamp: DateTime<Utc>,

    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub details: Option<serde_json::Value>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}