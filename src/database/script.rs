use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[sea_orm::model]
#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "script")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub title: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub content: Option<String>,
    pub edited_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl ActiveModelBehavior for ActiveModel {}
