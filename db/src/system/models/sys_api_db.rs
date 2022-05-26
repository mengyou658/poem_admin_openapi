use poem_openapi::{Object, types::ParseFromParameter, ApiRequest};
use serde::Deserialize;
use app_derive::QueryObject;

#[derive(Deserialize, Clone, Object)]
pub struct AddEditReq {
    pub api_id: String,
    pub dbs: Vec<String>,
}

#[derive(Deserialize, QueryObject, Clone, Object)]
pub struct SearchReq {
    pub api_id: String,
}
