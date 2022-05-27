use poem_openapi::Object;
use serde::Deserialize;
use app_derive::QueryObject;

#[derive(Deserialize, Debug, Default, Object, QueryObject)]
pub struct SearchReq {
    pub ip: Option<String>,
    pub user_name: Option<String>,
    pub status: Option<String>,
    pub order_by_column: Option<String>,
    pub is_asc: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Deserialize, Object, QueryObject)]
pub struct DeleteReq {
    pub info_ids: Vec<String>,
}
