use db::{
    common::res::Res,
    db_conn,
    system::{
        entities::sys_update_log,
        models::sys_update_log::{AddReq, DeleteReq, EditReq},
    },
    DB,
};
use poem_openapi::{OpenApi, param::Query, payload::Json};

use crate::utils::jwt::Claims;

use super::super::service;

pub struct SysUpdateLogApi;

#[OpenApi]
impl SysUpdateLogApi {

    /// add 添加
    #[oai(path="/system/update_log/add", method="post")]
    pub async fn add(&self, req: Json<AddReq>, user: Claims) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_update_log::add(db, req.0, &user.id).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    /// delete 完全删除
    #[oai(path="/system/update_log/delete", method="delete")]
    pub async fn delete(&self, req: Json<DeleteReq>) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_update_log::soft_delete(db, &req.id).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    // edit 修改
    #[oai(path="/system/update_log/edit", method="put")]
    pub async fn edit(&self, req: Json<EditReq>, user: Claims) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_update_log::edit(db, req.0, &user.id).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    /// get_all 获取全部
    #[oai(path="/system/update_log/get_all", method="get")]
    pub async fn get_all(&self) -> Res<Vec<sys_update_log::Model>> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_update_log::get_all(db).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
}
