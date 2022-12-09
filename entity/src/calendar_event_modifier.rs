//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5

use super::sea_orm_active_enums::Field;
use super::sea_orm_active_enums::Operation;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "calendar_event_modifier")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub calendar: Vec<u8>,
    pub field: Field,
    pub operation: Operation,
    pub value: String,
    pub new_value: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::calendar::Entity",
        from = "Column::Calendar",
        to = "super::calendar::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Calendar,
}

impl Related<super::calendar::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Calendar.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
