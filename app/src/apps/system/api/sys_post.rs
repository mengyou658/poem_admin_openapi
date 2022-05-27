use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        entities::sys_post,
        models::sys_post::{AddReq, DeleteReq, EditReq, Resp, SearchReq},
    },
    DB,
};
use poem::{
    handler,
};
use poem_openapi::{OpenApi, param::Query, payload::Json};

use super::super::service;
use crate::utils::jwt::Claims;

pub struct SysPostApi {}

#[OpenApi]
impl SysPostApi {

    /// get_list 获取列表
    /// page_params 分页参数
    #[oai(path="/system/post/get_sort_list", method="get")]
    pub async fn get_sort_list(&self, page_params: Query<PageParams>, req: Query<SearchReq>) -> Res<ListData<sys_post::Model>> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_post::get_sort_list(db, page_params.0, req.0).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }

    /// add 添加
    #[oai(path="/system/post/add", method="post")]
    pub async fn add(&self, req: Json<AddReq>, user: Claims) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_post::add(db, req.0, user.id).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }

    /// delete 完全删除
    #[oai(path="/system/post/delete", method="delete")]
    pub async fn delete(&self, req: Json<DeleteReq>) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_post::delete(db, req.0).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }

    // edit 修改
    #[oai(path="/system/post/edit", method="put")]
    pub async fn edit(&self, req: Json<EditReq>, user: Claims) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_post::edit(db, req.0, user.id).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }

    /// get_user_by_id 获取用户Id获取用户
    #[oai(path="/system/post/get_by_id", method="get")]
    pub async fn get_by_id(&self, req: Query<SearchReq>) -> Res<Resp> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_post::get_by_id(db, req.0).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }

    /// get_all 获取全部
    #[oai(path="/system/post/add", method="get")]
    pub async fn get_all(&self) -> Res<Vec<Resp>> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_post::get_all(db).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }


}
