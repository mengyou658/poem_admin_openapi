use poem_openapi::Object;
use serde::Deserialize;
use app_derive::QueryObject;

#[derive(Deserialize, Debug, Object, QueryObject)]
pub struct AddReq {
    pub role_id: String,
    pub api: String,
    pub method: Option<String>,
}
