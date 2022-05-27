use poem_openapi::Object;
use serde::Deserialize;
use app_derive::QueryObject;

#[derive(Deserialize, Clone, Debug, Object, QueryObject)]
pub struct SysDictSearchReq {
    pub dict_type_id: Option<String>,
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Object, QueryObject)]
pub struct SysDictAddReq {
    pub dict_name: String,
    pub dict_type: String,
    pub status: String,
    pub remark: Option<String>,
}

#[derive(Deserialize, Clone, Object, QueryObject)]
pub struct SysDictDeleteReq {
    pub dict_type_ids: Vec<String>,
}

#[derive(Deserialize, Clone, Debug, Object, QueryObject)]
pub struct SysDictEditReq {
    pub dict_type_id: String,
    pub dict_name: String,
    pub dict_type: String,
    pub status: String,
    pub remark: Option<String>,
}
