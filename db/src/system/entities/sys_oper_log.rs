//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_oper_log")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub oper_id: String,
    pub time_id: i64,
    pub title: String,
    pub business_type: String,
    pub method: String,
    pub request_method: String,
    pub operator_type: String,
    pub oper_name: String,
    pub dept_name: String,
    pub oper_url: String,
    pub oper_ip: String,
    pub oper_location: String,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".to_owned())")]
    pub oper_param: String,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".to_owned())")]
    pub url_param: String,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".to_owned())")]
    pub json_result: String,
    pub status: String,
    pub error_msg: String,
    pub oper_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
