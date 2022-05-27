use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        entities::sys_job,
        models::sys_job::{AddReq, DeleteReq, EditReq, JobId, SearchReq, StatusReq},
    },
    DB,
};
use poem_openapi::{OpenApi, param::Query, payload::Json};

use super::super::service;
use crate::{tasks, utils::jwt::Claims};

pub struct SysJobApi;

#[OpenApi]
impl SysJobApi {

    /// get_list 获取列表
    /// page_params 分页参数
    /// db 数据库连接 使用db.0
    #[oai(path="/system/job/get_sort_list", method="get")]
    pub async fn get_sort_list(&self, page_params: Query<PageParams>, req: Query<SearchReq>) -> Res<ListData<sys_job::Model>> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_job::get_sort_list(db, page_params.0, req.0).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    /// add 添加
    #[oai(path="/system/job/add", method="post")]
    pub async fn add(&self, req: Json<AddReq>, user: Claims) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_job::add(db, req.0, user.id).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    /// delete 完全删除
    #[oai(path="/system/job/delete", method="delete")]
    pub async fn delete(&self, req: Json<DeleteReq>) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_job::delete(db, req.0).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    // edit 修改
    #[oai(path="/system/job/edit", method="put")]
    pub async fn edit(&self, edit_req: Json<EditReq>, user: Claims) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_job::edit(db, edit_req.0, user.id).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    /// get_user_by_id 获取用户Id获取用户
    #[oai(path="/system/job/get_by_id", method="get")]
    pub async fn get_by_id(&self, req: Query<SearchReq>) -> Res<sys_job::Model> {
        let id = match req.0.job_id {
            None => return Res::with_err("id不能为空"),
            Some(x) => x,
        };
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_job::get_by_id(db, id).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    #[oai(path="/system/job/change_status", method="put")]
    pub async fn change_status(&self, req: Json<StatusReq>) -> Res<String> {
        //  数据验证
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_job::set_status(db, req.0).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    #[oai(path="/system/job/run_task_once", method="put")]
    pub async fn run_task_once(&self, req: Json<JobId>) -> Res<String> {
        tasks::run_once_task(req.0.job_id, req.0.task_id, true).await;
        Res::with_msg("任务开始执行")
    }
}
