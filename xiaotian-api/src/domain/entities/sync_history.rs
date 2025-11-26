use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sync_histories")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,

    #[sea_orm(column_type = "String(Some(100))", not_null)]
    pub sync_id: String,

    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub start_time: DateTime<Utc>,

    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub end_time: Option<DateTime<Utc>>,

    #[sea_orm(column_type = "Integer", nullable)]
    pub duration: Option<i32>, // in seconds

    #[sea_orm(column_type = "String(Some(20))", not_null)]
    pub status: String, // "completed", "failed", "cancelled"

    #[sea_orm(column_type = "Integer", not_null, default_value = "0")]
    pub feeds_processed: i32,

    #[sea_orm(column_type = "Integer", not_null, default_value = "0")]
    pub items_processed: i32,

    #[sea_orm(column_type = "Integer", not_null, default_value = "0")]
    pub summaries_generated: i32,

    #[sea_orm(column_type = "Boolean", not_null, default_value = "false")]
    pub email_sent: bool,

    #[sea_orm(column_type = "Integer", not_null, default_value = "0")]
    pub errors: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}