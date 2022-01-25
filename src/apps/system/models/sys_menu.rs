use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, Clone)]
pub struct SearchReq {
    #[validate(length(min = 1))]
    pub id: Option<String>,
    #[validate(length(min = 1))]
    pub menu_name: Option<String>,
    pub menu_type: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, FromQueryResult, Default)]
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
    pub perms: String,
    pub component: String,
    pub visible: String,
    pub is_frame: String,
    pub is_cache: String,
    pub is_data_scope: String,
    pub remark: String,
}

#[derive(Serialize, Clone, Validate, Debug, Default)]
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

#[derive(Serialize, Clone, Validate, Debug, Default)]
pub struct Meta {
    pub icon: String,
    pub title: String,
    pub link: Option<String>,
    // pub keep_alive: bool,
    pub no_cache: bool,
    pub hidden: bool,
}

#[derive(Serialize, Clone, Validate, Debug, Default)]
pub struct SysMenuTree {
    #[serde(flatten)]
    pub user_menu: UserMenu,
    pub children: Option<Vec<SysMenuTree>>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Validate)]
pub struct AddReq {
    pub pid: String,
    pub path: Option<String>,
    pub menu_name: String,
    pub icon: Option<String>,
    pub menu_type: String,
    pub query: Option<String>,
    pub order_sort: i32,
    pub status: String,
    pub perms: Option<String>,
    pub component: String,
    pub visible: String,
    pub is_frame: String,
    pub is_cache: String,
    pub is_data_scope: String,
    pub remark: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct DeleteReq {
    #[validate(length(min = 1, message = "至少要有一个id"))]
    pub ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
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
    pub perms: Option<String>,
    pub component: String,
    pub visible: String,
    pub is_frame: String,
    pub is_cache: String,
    pub is_data_scope: String,
    pub remark: String,
}