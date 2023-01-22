//! SeaORM Entity. Generated by sea-orm-codegen 0.9.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "addon")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub category_id: String,
    pub version: String,
    pub date: String,
    pub name: String,
    #[sea_orm(nullable)]
    pub author_name: Option<String>,
    #[sea_orm(nullable)]
    pub file_info_url: Option<String>,
    #[sea_orm(nullable)]
    pub download_total: Option<String>,
    #[sea_orm(nullable)]
    pub download_monthly: Option<String>,
    #[sea_orm(nullable)]
    pub favorite_total: Option<String>,
    #[sea_orm(nullable)]
    pub md5: Option<String>,
    #[sea_orm(nullable)]
    pub file_name: Option<String>,
    #[sea_orm(nullable)]
    pub download: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::installed_addon::Entity")]
    InstalledAddon,
    #[sea_orm(has_many = "super::addon_dir::Entity")]
    AddonDir,
    #[sea_orm(has_many = "super::addon_dependency::Entity")]
    AddonDependency,
}

impl Related<super::installed_addon::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InstalledAddon.def()
    }
}

impl Related<super::addon_dir::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AddonDir.def()
    }
}

impl Related<super::addon_dependency::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AddonDependency.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
