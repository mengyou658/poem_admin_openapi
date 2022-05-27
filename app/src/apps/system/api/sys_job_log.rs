use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        entities::sys_job_log,
        models::sys_job_log::{CleanReq, DeleteReq, SearchReq},
    },
    DB,
};
use poem::{
    handler,
};
use poem_openapi::{OpenApi, param::Query, payload::Json};

use super::super::service;

pub struct SysJobLogApi;

#[OpenApi]
impl SysJobLogApi {

    /// get_list 获取列表
    /// page_params 分页参数
    /// db 数据库连接 使用db.0
    #[oai(path="/system/job_log/get_sort_list", method="get")]
    pub async fn get_sort_list(&self, page_params: Query<PageParams>, req: Query<SearchReq>) -> Res<ListData<sys_job_log::Model>> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_job_log::get_sort_list(db, page_params.0, req.0).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    /// delete 完全删除
    #[oai(path="/system/job_log/delete", method="delete")]
    pub async fn delete(&self, req: Json<DeleteReq>) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_job_log::delete(db, req.0).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    #[oai(path="/system/job_log/clean", method="delete")]
    pub async fn clean(&self, req: Json<CleanReq>) -> Res<String> {
        //  数据验证
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_job_log::clean(db, req.0.job_id).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    /// get_user_by_id 获取用户Id获取用户
    /// db 数据库连接 使用db.0
    #[oai(path="/system/job_log/get_by_id", method="get")]
    pub async fn get_by_id(&self, req: Query<SearchReq>) -> Res<sys_job_log::Model> {
        let id = match req.0.job_log_id {
            None => return Res::with_err("id不能为空"),
            Some(x) => x,
        };
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_job_log::get_by_id(db, id).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
}
