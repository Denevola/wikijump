//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "site")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub site_id: i64,
    pub name: Option<String>,
    pub subtitle: Option<String>,
    #[sea_orm(column_type = "Text", unique)]
    pub slug: String,
    pub description: Option<String>,
    pub language: String,
    pub date_created: Option<DateTimeUtc>,
    pub custom_domain: Option<String>,
    pub visible: bool,
    pub default_page: String,
    pub private: bool,
    pub deleted: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::forum_group::Entity")]
    ForumGroup,
    #[sea_orm(has_many = "super::forum_category::Entity")]
    ForumCategory,
    #[sea_orm(has_many = "super::forum_post::Entity")]
    ForumPost,
    #[sea_orm(has_many = "super::forum_thread::Entity")]
    ForumThread,
    #[sea_orm(has_many = "super::site_settings::Entity")]
    SiteSettings,
    #[sea_orm(has_many = "super::user_block::Entity")]
    UserBlock,
    #[sea_orm(has_many = "super::page::Entity")]
    Page,
    #[sea_orm(has_many = "super::page_connection_missing::Entity")]
    PageConnectionMissing,
    #[sea_orm(has_many = "super::page_category::Entity")]
    PageCategory,
    #[sea_orm(has_many = "super::page_revision::Entity")]
    PageRevision,
}

impl Related<super::forum_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ForumGroup.def()
    }
}

impl Related<super::forum_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ForumCategory.def()
    }
}

impl Related<super::forum_post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ForumPost.def()
    }
}

impl Related<super::forum_thread::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ForumThread.def()
    }
}

impl Related<super::site_settings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SiteSettings.def()
    }
}

impl Related<super::user_block::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserBlock.def()
    }
}

impl Related<super::page::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Page.def()
    }
}

impl Related<super::page_connection_missing::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PageConnectionMissing.def()
    }
}

impl Related<super::page_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PageCategory.def()
    }
}

impl Related<super::page_revision::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PageRevision.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
