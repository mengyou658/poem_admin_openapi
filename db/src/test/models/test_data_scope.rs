use app_derive::QueryObject;
use poem_openapi::Object;
use serde::Deserialize;
#[derive(Deserialize, Debug, Object, QueryObject)]
pub struct SearchReq {
    pub data_a: Option<String>,
    pub data_b: Option<String>,
}

#[derive(Deserialize, Debug, Object, QueryObject)]
pub struct AddReq {
    pub data_a: String,
    pub data_b: String,
}

#[derive(Deserialize, Object, QueryObject)]
pub struct DeleteReq {
    pub ids: Vec<String>,
}
