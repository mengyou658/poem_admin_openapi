//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "sys_menu"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub pid: String,
    pub path: String,
    pub menu_name: String,
    pub icon: String,
    pub menu_type: String,
    pub query: Option<String>,
    pub order_sort: i32,
    pub status: String,
    pub api: String,
    pub method: String,
    pub component: String,
    pub visible: String,
    pub is_cache: String,
    pub is_frame: String,
    pub is_data_scope: String,
    pub remark: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Pid,
    Path,
    MenuName,
    Icon,
    MenuType,
    Query,
    OrderSort,
    Status,
    Api,
    Method,
    Component,
    Visible,
    IsCache,
    IsFrame,
    IsDataScope,
    Remark,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = String;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::String(Some(32u32)).def(),
            Self::Pid => ColumnType::String(Some(32u32)).def(),
            Self::Path => ColumnType::String(Some(255u32)).def(),
            Self::MenuName => ColumnType::String(Some(100u32)).def(),
            Self::Icon => ColumnType::String(Some(50u32)).def(),
            Self::MenuType => ColumnType::Char(Some(1u32)).def(),
            Self::Query => ColumnType::String(Some(255u32)).def().null(),
            Self::OrderSort => ColumnType::Integer.def(),
            Self::Status => ColumnType::Char(Some(1u32)).def(),
            Self::Api => ColumnType::String(Some(155u32)).def().unique(),
            Self::Method => ColumnType::String(Some(10u32)).def(),
            Self::Component => ColumnType::String(Some(100u32)).def(),
            Self::Visible => ColumnType::Char(Some(1u32)).def(),
            Self::IsCache => ColumnType::Char(Some(1u32)).def(),
            Self::IsFrame => ColumnType::Char(Some(1u32)).def(),
            Self::IsDataScope => ColumnType::Char(Some(1u32)).def(),
            Self::Remark => ColumnType::String(Some(255u32)).def(),
            Self::CreatedAt => ColumnType::DateTime.def().null(),
            Self::UpdatedAt => ColumnType::DateTime.def().null(),
            Self::DeletedAt => ColumnType::DateTime.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
