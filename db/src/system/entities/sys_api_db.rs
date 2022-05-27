//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use app_derive::QueryObject;
use poem_openapi::Object;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "sys_api_db"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize, Object, QueryObject)]
pub struct Model {
    pub api_id: String,
    pub db: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    ApiId,
    Db,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    ApiId,
    Db,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = (String, String);
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
            Self::ApiId => ColumnType::String(Some(32u32)).def(),
            Self::Db => ColumnType::String(Some(32u32)).def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
