use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        entities::sys_login_log,
        models::sys_login_log::{DeleteReq, SearchReq},
    },
    DB,
};
use poem_openapi::{OpenApi, param::Query, payload::Json};

use super::super::service;

pub struct SysLoginLogApi;

#[OpenApi]
impl SysLoginLogApi {

    /// get_list 获取列表
    /// page_params 分页参数
    #[oai(path="/system/login-log/get_sort_list", method="get")]
    pub async fn get_sort_list(&self, page_params: Query<PageParams>, req: Query<SearchReq>) -> Res<ListData<sys_login_log::Model>> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_login_log::get_sort_list(db, page_params.0, req.0).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    #[oai(path="/system/login-log/delete", method="delete")]
    pub async fn delete(&self, delete_req: Json<DeleteReq>) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_login_log::delete(db, delete_req.0).await;
        println!("{:?}", res);
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    #[oai(path="/system/login-log/clean", method="delete")]
    pub async fn clean(&self) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_login_log::clean(db).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
}
