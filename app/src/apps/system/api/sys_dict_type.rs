use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
      entities::sys_dict_type,
      models::sys_dict_type::{SysDictAddReq, SysDictDeleteReq, SysDictEditReq, SysDictSearchReq},
    },
    DB,
};

use poem_openapi::{OpenApi, param::Query, payload::Json};

use super::super::service;
use crate::utils::jwt::Claims;


pub struct SysDictType;

#[OpenApi]
impl SysDictType {
    /// get_list 获取列表
    /// page_params 分页参数
    #[oai(path="/system/dict/type/get_sort_list", method="get")]
    pub async fn get_sort_list(&self, page_params: Query<Option<PageParams>>, req: Query<Option<SysDictSearchReq>>) -> Res<ListData<sys_dict_type::Model>> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_dict_type::get_sort_list(db, page_params.0.unwrap(), req.0.unwrap()).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    /// add 添加
    #[oai(path="/system/dict/type/add", method="post")]
    pub async fn add(&self, req: Json<SysDictAddReq>, user: Claims) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_dict_type::add(db, req.0, user.id).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }

    /// delete 完全删除
    #[oai(path="/system/dict/type/delete", method="delete")]
    pub async fn delete(&self, req: Json<SysDictDeleteReq>) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_dict_type::delete(db, req.0).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }

    // edit 修改
    #[oai(path="/system/dict/type/edit", method="put")]
    pub async fn edit(&self, edit_req: Json<SysDictEditReq>, user: Claims) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_dict_type::edit(db, edit_req.0, user.id).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }

    /// get_user_by_id 获取用户Id获取用户
    #[oai(path="/system/dict/type/get_by_id", method="get")]
    pub async fn get_by_id(&self, req: Query<Option<SysDictSearchReq>>) -> Res<sys_dict_type::Model> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_dict_type::get_by_id(db, req.0.unwrap()).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }

    /// get_all 获取全部

    #[oai(path="/system/dict/type/get_all", method="get")]
    pub async fn get_all(&self) -> Res<Vec<sys_dict_type::Model>> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_dict_type::get_all(db).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }

}
