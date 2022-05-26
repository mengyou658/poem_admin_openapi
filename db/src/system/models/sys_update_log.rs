use poem_openapi::Object;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Object)]
pub struct AddReq {
    pub app_version: String,
    pub backend_version: String,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize, Clone, Debug, Object)]
pub struct EditReq {
    pub id: String,
    pub app_version: String,
    pub backend_version: String,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize, Object)]
pub struct DeleteReq {
    pub id: String,
}
