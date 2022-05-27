use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    test::{
        entities::test_data_scope,
        models::test_data_scope::{AddReq, DeleteReq, SearchReq},
    },
    DB,
};
use poem_openapi::{OpenApi, param::Query, payload::Json};

use super::super::service;
use crate::utils::jwt::Claims;

pub struct SysTestDataScopeApi;

#[OpenApi]
impl SysTestDataScopeApi {

    /// get_list 获取列表
    /// page_params 分页参数
    /// db 数据库连接 使用db.0
    #[oai(path="/test/data_scope/get_sort_list", method="get")]
    pub async fn get_sort_list(&self, page_params: Query<PageParams>, req: Query<SearchReq>, user: Claims) -> Res<ListData<test_data_scope::Model>> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::test_data_scope::get_sort_list(db, page_params.0, req.0, &user.id).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    /// add 添加
    #[oai(path="/test/data_scope/add", method="post")]
    pub async fn add(&self, req: Json<AddReq>, user: Claims) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::test_data_scope::add(db, req.0, &user.id).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    /// delete 完全删除
    #[oai(path="/test/data_scope/delete", method="delete")]
    pub async fn delete(&self, req: Json<DeleteReq>) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::test_data_scope::delete(db, req.0).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
}
