use poem_openapi::Object;
use serde::Deserialize;

#[derive(Deserialize, Debug, Object)]
pub struct AddReq {
    pub role_id: String,
    pub api: String,
    pub method: Option<String>,
}
