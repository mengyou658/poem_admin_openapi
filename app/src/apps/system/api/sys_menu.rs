use configs::CFG;
use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        entities::sys_menu,
        models::sys_menu::{AddReq, DeleteReq, EditReq, LogCacheEditReq, MenuRelated, MenuResp, SearchReq, SysMenuTree},
    },
    DB,
};
use poem_openapi::{OpenApi, param::Query, payload::Json};

use super::super::service;
use crate::utils::jwt::Claims;

pub struct SysMenuApi;

#[OpenApi]
impl SysMenuApi {

    /// get_all_menu_tree 获取全部菜单
    #[oai(path="/system/menu/get_sort_list", method="get")]
    pub async fn get_sort_list(&self, page_params: Query<PageParams>, search_req: Query<SearchReq>) -> Res<ListData<sys_menu::Model>> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_menu::get_sort_list(db, page_params.0, search_req.0).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    /// get_user_by_id 获取用户Id获取用户
    /// db 数据库连接 使用db.0
    #[oai(path="/system/menu/get_by_id", method="get")]
    pub async fn get_by_id(&self, search_req: Query<SearchReq>) -> Res<MenuResp> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_menu::get_by_id(db, search_req.0).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    /// add 添加
    #[oai(path="/system/menu/add", method="post")]
    pub async fn add(&self, req: Json<AddReq>) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_menu::add(db, req.0).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    /// delete 完全删除
    #[oai(path="/system/menu/delete", method="delete")]
    pub async fn delete(&self, req: Json<DeleteReq>) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_menu::delete(db, &req.0.id).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    // edit 修改
    #[oai(path="/system/menu/edit", method="put")]
    pub async fn edit(&self, edit_req: Json<EditReq>) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_menu::edit(db, edit_req.0).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    // update_log_cache_method 修改菜单日志缓存方法
    #[oai(path="/system/menu/update_log_cache_method", method="put")]
    pub async fn update_log_cache_method(&self, edit_req: Json<LogCacheEditReq>) -> Res<String> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_menu::update_log_cache_method(db, edit_req.0).await;
        match res {
            Ok(x) => Res::with_msg(&x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    /// get_all_menu_tree 获取全部菜单树
    #[oai(path="/system/menu/get_all_enabled_menu_tree", method="get")]
    pub async fn get_all_enabled_menu_tree(&self) -> Res<Vec<SysMenuTree>> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_menu::get_all_enabled_menu_tree(db).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    /// get_related_api_and_db 获取全部菜单树
    #[oai(path="/system/menu/get_related_api_and_db", method="get")]
    pub async fn get_related_api_and_db(&self, page_params: Query<PageParams>, search_req: Query<SearchReq>) -> Res<ListData<MenuRelated>> {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_menu::get_related_api_and_db(db, page_params.0, search_req.0).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
    
    /// 获取用户路由
    #[oai(path="/system/menu/get_routers", method="get")]
    pub async fn get_routers(&self, user: Claims) -> Res<Vec<SysMenuTree>> {
        let db = DB.get_or_init(db_conn).await;
        //  获取 用户角色
        let role_id = match service::sys_role::get_current_admin_role(db, &user.id).await {
            Ok(x) => x,
            Err(e) => return Res::with_err(&e.to_string()),
        };
    
        // 检查是否超管用户
        let res = if CFG.system.super_user.contains(&user.id) {
            service::sys_menu::get_all_router_tree(db).await
        } else {
            service::sys_menu::get_admin_menu_by_role_ids(db, &role_id).await
        };
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    }
}
