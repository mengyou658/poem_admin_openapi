use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        entities::sys_dept,
        models::sys_dept::{AddReq, DeleteReq, DeptResp, EditReq, RespTree, SearchReq},
    },
    DB,
};
use poem_openapi::{OpenApi, param::Query, payload::Json};

use super::super::service;
use crate::utils::jwt::Claims;

pub struct SysDeptApi;

#[OpenApi]
impl SysDeptApi {
    
    
    /// get_list 获取列表
    /// page_params 分页参数
    /// db 数据库连接 使用db.0
    #[oai(path="/system/dept/get_sort_list", method="get")]
    pub async fn get_sort_list(&self, page_params: Query<PageParams>, req: Query<SearchReq>) -> Res<ListData<sys_dept::Model>> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_dept::get_sort_list(db, page_params.0, req.0).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    /// add 添加
    #[oai(path="/system/dept/add", method="post")]
    pub async fn add(&self, req: Json<AddReq>, user: Claims) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_dept::add(db, req.0, user.id).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    /// delete 完全删除
    #[oai(path="/system/dept/delete", method="delete")]
    pub async fn delete(&self, req: Json<DeleteReq>) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_dept::delete(db, req.0).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    // edit 修改
    #[oai(path="/system/dept/edit", method="put")]
    pub async fn edit(&self, req: Json<EditReq>, user: Claims) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_dept::edit(db, req.0, user.id).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    /// get_user_by_id 获取用户Id获取用户
    /// db 数据库连接 使用db.0
    #[oai(path="/system/dept/get_by_id", method="get")]
    pub async fn get_by_id(&self, req: Query<SearchReq>) -> Res<DeptResp> {
        let db = DB.get_or_init(db_conn).await;
        if let Some(x) = req.0.dept_id {
            let res = service::sys_dept::get_by_id(db, &x).await;
            match res {
                Ok(x) => Res::with_data(x),
                Err(e) => Res::with_err(&e.to_string()),
            }
        } else {
            Res::with_err("参数错误")
        }
    }
    
    /// get_all 获取全部
    #[oai(path="/system/dept/get_all", method="get")]
    pub async fn get_all(&self) -> Res<Vec<DeptResp>> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_dept::get_all(db).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    #[oai(path="/system/dept/get_dept_tree", method="get")]
    pub async fn get_dept_tree(&self) -> Res<Vec<RespTree>> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_dept::get_dept_tree(db).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
}
