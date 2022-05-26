use poem_openapi::Object;
use serde::Deserialize;
#[derive(Deserialize, Debug, Object)]
pub struct SearchReq {
    pub job_id: Option<String>,
    pub job_name: Option<String>,
    pub job_group: Option<String>,
    pub status: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Object)]
pub struct AddReq {
    pub task_id: i64,
    pub task_count: i64,
    pub job_name: String,
    pub job_params: Option<String>,
    pub job_group: String,
    pub invoke_target: String,
    pub cron_expression: String,
    pub misfire_policy: String,
    pub concurrent: Option<String>,
    pub status: Option<String>,
    pub remark: Option<String>,
}

#[derive(Deserialize, Object)]
pub struct DeleteReq {
    pub job_ids: Vec<String>,
}

#[derive(Deserialize, Debug, Object)]
pub struct EditReq {
    pub job_id: String,
    pub task_id: i64,
    pub task_count: i64,
    pub job_name: String,
    pub job_params: Option<String>,
    pub job_group: String,
    pub invoke_target: String,
    pub cron_expression: String,
    pub misfire_policy: String,
    pub concurrent: Option<String>,
    pub status: Option<String>,
    pub remark: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Object)]
pub struct StatusReq {
    pub job_id: String,
    pub status: String,
}

#[derive(Deserialize, Clone, Debug, Object)]
pub struct JobId {
    pub job_id: String,
    pub task_id: i64,
}
