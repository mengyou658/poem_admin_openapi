use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        entities::sys_user_online,
        models::sys_user_online::{DeleteReq, SearchReq},
    },
    DB,
};
use poem_openapi::{OpenApi, param::Query, payload::Json};

use super::super::service;
use crate::utils::jwt::Claims;

pub struct SysUserOnlineApi;

#[OpenApi]
impl SysUserOnlineApi {
        
        /// get_list 获取列表
        /// page_params 分页参数
        #[oai(path="/system/online/get_sort_list", method="get")]
        pub async fn get_sort_list(&self, page_params: Query<PageParams>, req: Query<SearchReq>) -> Res<ListData<sys_user_online::Model>> {
            let db = DB.get_or_init(db_conn).await;
            let res = service::sys_user_online::get_sort_list(db, page_params.0, req.0).await;
            match res {
                Ok(x) => Res::with_data(x),
                Err(e) => Res::with_err(&e.to_string()),
            }
        }
        
        #[oai(path="/system/online/delete", method="delete")]
        pub async fn delete(&self, delete_req: Json<DeleteReq>) -> Res<String> {
            let db = DB.get_or_init(db_conn).await;
            let res = service::sys_user_online::delete(db, delete_req.0).await;
            match res {
                Ok(x) => Res::with_msg(&x),
                Err(e) => Res::with_err(&e.to_string()),
            }
        }
        
        #[oai(path="/system/log_out", method="post")]
        pub async fn log_out(&self, user: Claims) -> Res<String> {
            let db = DB.get_or_init(db_conn).await;
            let res = service::sys_user_online::log_out(db, user.token_id).await;
            match res {
                Ok(x) => Res::with_msg(&x),
                Err(e) => Res::with_err(&e.to_string()),
            }
        }
}
