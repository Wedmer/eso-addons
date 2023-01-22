//! SeaORM Entity. Generated by sea-orm-codegen 0.9.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "installed_addon")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub addon_id: i32,
    pub version: String,
    pub date: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::addon::Entity",
        from = "Column::AddonId",
        to = "super::addon::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Addon,
}

impl Related<super::addon::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Addon.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
