use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "task_executions")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,

    #[sea_orm(column_type = "String(Some(100))", not_null)]
    pub execution_id: String,

    #[sea_orm(column_type = "String(Some(100))", not_null)]
    pub task_id: String,

    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub start_time: DateTime<Utc>,

    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub end_time: Option<DateTime<Utc>>,

    #[sea_orm(column_type = "Integer", nullable)]
    pub duration: Option<i32>, // in seconds

    #[sea_orm(column_type = "String(Some(20))", not_null)]
    pub status: String, // "success", "failed"

    #[sea_orm(column_type = "Integer", not_null, default_value = "0")]
    pub feeds_processed: i32,

    #[sea_orm(column_type = "Integer", not_null, default_value = "0")]
    pub summaries_generated: i32,

    #[sea_orm(column_type = "Boolean", not_null, default_value = "false")]
    pub email_sent: bool,

    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub logs: Option<serde_json::Value>, // Array of log strings
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::scheduled_task::Entity",
        from = "Column::TaskId",
        to = "super::scheduled_task::Column::Id"
    )]
    ScheduledTask,
}

impl Related<super::scheduled_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ScheduledTask.def()
    }
}