use poem_openapi::Object;
use serde::Deserialize;

#[derive(Deserialize, Debug, Object)]
pub struct SearchReq {
    pub oper_id: Option<String>,
    pub title: Option<String>,
    pub oper_name: Option<String>,
    pub operator_type: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Object)]
pub struct DeleteReq {
    pub oper_log_ids: Vec<String>,
}
