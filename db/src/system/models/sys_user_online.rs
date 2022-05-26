use poem_openapi::Object;
use serde::Deserialize;

#[derive(Deserialize, Debug, Object)]
pub struct SearchReq {
    pub ipaddr: Option<String>,
    pub user_name: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Deserialize, Object)]
pub struct DeleteReq {
    pub ids: Vec<String>,
}
