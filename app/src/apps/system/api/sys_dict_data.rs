use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
      entities::sys_dict_data,
      models::sys_dict_data::{DictDataAddReq, DictDataDeleteReq, DictDataEditReq, DictDataSearchReq},
    },
    DB,
};
use poem::{
    handler,
    web::{Json, Query},
    IntoResponse,
};
use poem_openapi::OpenApi;

use super::super::service;
use crate::utils::jwt::Claims;

pub struct SysDictData;

#[OpenApi]
impl SysDictData {

    /// get_list 获取列表
    /// page_params 分页参数
    /// db 数据库连接 使用db.0
    #[oai(path="/system/dict/data/get_sort_list", method="get")]
    pub async fn get_sort_list(&self, page_params: Query<PageParams>, req: Query<DictDataSearchReq>) -> Res<ListData<sys_dict_data::DictDataModel>> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_dict_data::get_sort_list(db, page_params.0, req.0).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }

    /// add 添加
    #[oai(path="/system/dict/data/add", method="post")]
    pub async fn add(&self, req: Json<DictDataAddReq>, user: Claims) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_dict_data::add(db, req.0, user.id).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }

    /// delete 完全删除
    #[oai(path="/system/dict/data/delete", method="delete")]
    pub async fn delete(&self, req: Json<DictDataDeleteReq>) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_dict_data::delete(db, req.0).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }

    // edit 修改
    #[oai(path="/system/dict/data/edit", method="put")]
    pub async fn edit(&self, req: Json<DictDataEditReq>, user: Claims) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_dict_data::edit(db, req.0, user.id).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }

    /// get_user_by_id 获取用户Id获取用户
    /// db 数据库连接 使用db.0
    #[oai(path="/system/dict/data/get_by_id", method="get")]
    pub async fn get_by_id(&self, req: Query<DictDataSearchReq>) -> Res<sys_dict_data::DictDataModel> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_dict_data::get_by_id(db, req.0).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }

    /// get_user_by_id 获取用户Id获取用户
    /// db 数据库连接 使用db.0
    #[oai(path="/system/dict/data/get_by_type", method="get")]
    pub async fn get_by_type(&self, req: Query<DictDataSearchReq>) -> Res<Vec<sys_dict_data::DictDataModel>> {
        let db = DB.get_or_init(db_conn).await;
        match service::sys_dict_data::get_by_type(db, req.0).await {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }

    /// get_all 获取全部
    /// db 数据库连接 使用db.0
    #[oai(path="/system/dict/data/get_all", method="get")]
    pub async fn get_all(&self) -> Res<Vec<sys_dict_data::DictDataModel>> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_dict_data::get_all(db).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }

}
