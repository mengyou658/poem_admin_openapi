use poem_openapi::Object;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use crate::system::entities::sys_menu;

#[derive(Deserialize, Clone, Object)]
pub struct SearchReq {
    pub id: Option<String>,
    pub menu_name: Option<String>,
    pub menu_type: Option<String>,
    pub method: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, FromQueryResult, Default, Object)]
pub struct MenuResp {
    pub id: String,
    pub pid: String,
    pub path: String,
    pub menu_name: String,
    pub icon: String,
    pub menu_type: String,
    pub query: Option<String>,
    pub order_sort: i32,
    pub status: String,
    pub api: String,
    pub method: String,
    pub component: String,
    pub visible: String,
    pub is_frame: String,
    pub is_cache: String,
    pub data_scope: String,
    pub log_method: String,
    pub data_cache_method: String,
    pub remark: String,
}

#[derive(Serialize, Clone, Debug, Object)]
pub struct MenuRelated {
    #[serde(flatten)]
    pub menu: sys_menu::Model,
    pub dbs: Vec<String>,
    pub apis: Vec<String>,
}

#[derive(Serialize, Clone, Debug, Default, Object)]
pub struct UserMenu {
    pub id: String,
    pub pid: String,
    pub always_show: Option<bool>,
    pub path: String,
    pub name: String,
    pub menu_name: String,
    pub menu_type: String,
    pub component: String,
    pub hidden: bool,
    pub meta: Meta,
}

#[derive(Serialize, Clone, Debug, Default, Object)]
pub struct Meta {
    pub icon: String,
    pub title: String,
    pub link: Option<String>,
    pub no_cache: bool,
    pub hidden: bool,
}

#[derive(Serialize, Clone, Debug, Default, Object)]
pub struct SysMenuTree {
    #[serde(flatten)]
    pub user_menu: UserMenu,
    pub children: Option<Vec<SysMenuTree>>,
}

#[derive(Deserialize, Clone, Debug, Object)]
pub struct AddReq {
    pub pid: String,
    pub path: Option<String>,
    pub menu_name: String,
    pub icon: Option<String>,
    pub menu_type: String,
    pub query: Option<String>,
    pub order_sort: i32,
    pub status: String,
    pub api: String,
    pub method: Option<String>,
    pub component: Option<String>,
    pub visible: String,
    pub is_frame: String,
    pub is_cache: String,
    pub data_scope: String,
    pub log_method: String,
    pub data_cache_method: String,
    pub remark: String,
}

#[derive(Debug, Deserialize, Object)]
pub struct DeleteReq {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, Object)]
pub struct EditReq {
    pub id: String,
    pub pid: String,
    pub path: String,
    pub menu_name: String,
    pub icon: Option<String>,
    pub menu_type: String,
    pub query: Option<String>,
    pub order_sort: i32,
    pub status: String,
    pub api: String,
    pub method: Option<String>,
    pub component: String,
    pub visible: String,
    pub is_frame: String,
    pub is_cache: String,
    pub data_scope: String,
    pub log_method: String,
    pub data_cache_method: String,
    pub remark: String,
}

#[derive(Debug, Clone, Deserialize, Object)]
pub struct LogCacheEditReq {
    pub id: String,
    pub log_method: String,
    pub data_cache_method: String,
}
