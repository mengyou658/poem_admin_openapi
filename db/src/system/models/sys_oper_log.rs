use poem_openapi::Object;
use serde::Deserialize;
use app_derive::QueryObject;

#[derive(Deserialize, Debug, Object, QueryObject)]
pub struct SearchReq {
    pub oper_id: Option<String>,
    pub title: Option<String>,
    pub oper_name: Option<String>,
    pub operator_type: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Object, QueryObject)]
pub struct DeleteReq {
    pub oper_log_ids: Vec<String>,
}
