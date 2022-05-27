use poem_openapi::Object;
use serde::Deserialize;
use app_derive::QueryObject;

#[derive(Deserialize, Debug, Object, QueryObject)]
pub struct DictDataSearchReq {
    pub dict_data_id: Option<String>,
    pub dict_type: Option<String>,
    pub dict_label: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Default, Deserialize, Clone, Debug, Object, QueryObject)]
pub struct DictDataAddReq {
    pub dict_type: String,
    pub dict_label: String,
    pub dict_value: String,
    pub dict_sort: i32,
    pub css_class: Option<String>,
    pub list_class: Option<String>,
    pub is_default: String,
    pub status: String,
    pub remark: Option<String>,
}

#[derive(Deserialize, Object, QueryObject)]
pub struct DictDataDeleteReq {
    pub dict_data_ids: Vec<String>,
}

#[derive(Deserialize, Debug, Object, QueryObject)]
pub struct DictDataEditReq {
    pub dict_data_id: String,
    pub dict_type: String,
    pub dict_label: String,
    pub dict_value: String,
    pub dict_sort: i32,
    pub css_class: Option<String>,
    pub list_class: Option<String>,
    pub is_default: String,
    pub status: String,
    pub remark: Option<String>,
}
