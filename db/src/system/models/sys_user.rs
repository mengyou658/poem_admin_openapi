use app_derive::QueryObject;
use chrono::NaiveDateTime;
use poem_openapi::Object;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use super::sys_dept::DeptResp;

#[derive(Deserialize, Debug, Object, QueryObject)]
pub struct AddReq {
    pub user_name: String,
    pub user_nickname: String,
    pub user_password: String,
    pub user_status: String,
    pub user_email: Option<String>,
    pub sex: String,
    pub avatar: Option<String>,
    pub remark: Option<String>,
    pub is_admin: String,
    pub phone_num: Option<String>,
    pub post_ids: Vec<String>,
    pub dept_ids: Vec<String>,
    pub dept_id: String,
    pub role_ids: Vec<String>,
    pub role_id: String,
}

#[derive(Deserialize, Debug, Object, QueryObject)]
pub struct EditReq {
    pub id: String,
    pub user_name: String,
    pub user_nickname: String,
    pub user_status: String,
    pub user_email: Option<String>,
    pub sex: String,
    pub avatar: String,
    pub remark: Option<String>,
    pub is_admin: String,
    pub phone_num: Option<String>,
    pub post_ids: Vec<String>,
    pub dept_ids: Vec<String>,
    pub dept_id: String,
    pub role_ids: Vec<String>,
    pub role_id: String,
}

#[derive(Deserialize, Debug, Object, QueryObject)]
pub struct UpdateProfileReq {
    pub id: String,
    pub user_nickname: String,
    pub phone_num: String,
    pub user_email: String,
    pub sex: String,
}

#[derive(Debug, Clone, Default, Serialize, FromQueryResult, Deserialize, Object, QueryObject)]
pub struct UserResp {
    pub id: String,
    pub user_name: String,
    pub user_nickname: String,
    pub user_status: String,
    pub user_email: Option<String>,
    pub sex: String,
    pub avatar: String,
    pub dept_id: String,
    pub remark: Option<String>,
    pub is_admin: String,
    pub phone_num: Option<String>,
    pub role_id: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Object, Deserialize, QueryObject)]
pub struct UserWithDept {
    #[serde(flatten)]
    pub user: UserResp,
    pub dept: DeptResp,
}

#[derive(Debug, Serialize, Object, Deserialize, QueryObject)]
pub struct UserInfomaion {
    pub user_info: UserWithDept,
    pub post_ids: Vec<String>,
    pub role_ids: Vec<String>,
    pub dept_ids: Vec<String>,
    pub dept_id: String,
}

#[derive(Deserialize, Debug, Object, QueryObject)]
pub struct SearchReq {
    pub user_id: Option<String>,
    pub role_id: Option<String>,
    pub user_ids: Option<Vec<String>>,
    pub user_name: Option<String>,
    pub phone_num: Option<String>,
    pub user_nickname: Option<String>,
    pub user_status: Option<String>,
    pub dept_id: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Object, QueryObject)]
pub struct DeleteReq {
    pub user_ids: Vec<String>,
}

///  ????????????
#[derive(Deserialize, Debug, Object, QueryObject)]
pub struct UserLoginReq {
    ///  ?????????
    pub user_name: String,
    ///  ????????????
    pub user_password: String,
    pub code: String,
    pub uuid: String,
}

#[derive(Serialize, Debug, Object, Deserialize, QueryObject)]
pub struct UserInfo {
    pub user: UserWithDept,
    pub roles: Vec<String>,
    pub depts: Vec<String>,
    pub permissions: Vec<String>,
}
#[derive(Deserialize, Object, QueryObject)]
pub struct ResetPwdReq {
    pub user_id: String,
    pub new_passwd: String,
}

#[derive(Deserialize, Object, QueryObject)]
pub struct UpdatePwdReq {
    pub old_passwd: String,
    pub new_passwd: String,
}

#[derive(Deserialize, Clone, Object, QueryObject)]
pub struct ChangeStatusReq {
    pub user_id: String,
    pub status: String,
}

#[derive(Deserialize, Clone, Object, QueryObject)]
pub struct ChangeRoleReq {
    pub user_id: String,
    pub role_id: String,
}

#[derive(Deserialize, Clone, Object, QueryObject)]
pub struct ChangeDeptReq {
    pub user_id: String,
    pub dept_id: String,
}
