use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "related_summaries")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,

    #[sea_orm(column_type = "Integer", not_null)]
    pub summary_id: i32,

    #[sea_orm(column_type = "Integer", not_null)]
    pub related_summary_id: i32,

    #[sea_orm(column_type = "Float", not_null)]
    pub relevance_score: f64,

    #[sea_orm(column_type = "String(Some(20))", not_null)]
    pub relation_type: String, // "content", "temporal", "source", "tag"

    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub shared_tags: Option<serde_json::Value>, // Array of strings

    #[sea_orm(column_type = "String(Some(1000))", nullable)]
    pub excerpt: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::summary::Entity",
        from = "Column::SummaryId",
        to = "super::summary::Column::Id"
    )]
    Summary,
}

impl Related<super::summary::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Summary.def()
    }
}