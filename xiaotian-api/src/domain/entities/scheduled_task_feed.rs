use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "scheduled_task_feeds")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,

    #[sea_orm(column_type = "String(Some(100))", not_null)]
    pub task_id: String,

    #[sea_orm(column_type = "Integer", not_null)]
    pub feed_id: i32,

    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::scheduled_task::Entity",
        from = "Column::TaskId",
        to = "super::scheduled_task::Column::Id"
    )]
    ScheduledTask,

    #[sea_orm(
        belongs_to = "super::feed::Entity",
        from = "Column::FeedId",
        to = "super::feed::Column::Id"
    )]
    Feed,
}

impl Related<super::scheduled_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ScheduledTask.def()
    }
}

impl Related<super::feed::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Feed.def()
    }
}