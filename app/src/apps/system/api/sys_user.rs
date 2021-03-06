use anyhow::Result;
use configs::CFG;
use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::models::sys_user::{
        AddReq, ChangeDeptReq, ChangeRoleReq, ChangeStatusReq, DeleteReq, EditReq, ResetPwdReq, SearchReq, UpdateProfileReq, UpdatePwdReq, UserInfo, UserInfomaion, UserLoginReq,
        UserWithDept,
    },
    DB,
};
use poem::{
    handler,
    web::{Json, Multipart, Query},
    Request,
};
use poem_openapi::OpenApi;
use tokio::join;

use super::super::service;
use crate::utils::jwt::{AuthBody, Claims};

pub struct SysUserApi;

#[OpenApi]
impl SysUserApi {

  /// get_user_list 获取用户列表
  /// page_params 分页参数
  #[oai(path="/system/user/get_sort_list", method="get")]
  pub async fn get_sort_list(&self, page_params: Query<PageParams>, req: Query<SearchReq>) -> Res<ListData<UserWithDept>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::get_sort_list(db, page_params.0, req.0).await;
    match res {
      Ok(x) => Res::with_data(x),
      Err(e) => Res::with_err(&e.to_string()),
    }
  }

  /// get_user_by_id 获取用户Id获取用户

  #[oai(path="/system/user/get_by_id", method="get")]
  pub async fn get_by_id(&self, req: Query<SearchReq>) -> Res<UserInfomaion> {
    match req.0.user_id {
      Some(user_id) => match Self::get_user_info_by_id(&user_id).await {
        Err(e) => Res::with_err(&e.to_string()),
        Ok(res) => Res::with_data(res),
      },
      None => Res::with_msg("用户id不能为空"),
    }
  }

  #[oai(path="/system/user/get_profile", method="get")]
  pub async fn get_profile(&self, user: Claims) -> Res<UserInfomaion> {
    match Self::get_user_info_by_id(&user.id).await {
      Err(e) => Res::with_err(&e.to_string()),
      Ok(res) => Res::with_data(res),
    }
  }

  pub async fn get_user_info_by_id(id: &str) -> Result<UserInfomaion> {
    let db = DB.get_or_init(db_conn).await;
    match service::sys_user::get_by_id(db, id).await {
      Err(e) => Err(e),
      Ok(user) => {
        let post_ids = service::sys_post::get_post_ids_by_user_id(db, &user.user.id).await.unwrap();
        let role_ids = service::sys_user_role::get_role_ids_by_user_id(db, &user.user.id).await.expect("角色id获取失败");
        let dept_ids = service::sys_user_dept::get_dept_ids_by_user_id(db, &user.user.id).await.expect("角色id获取失败");
        let res = UserInfomaion {
          user_info: user.clone(),
          dept_id: user.user.dept_id,
          post_ids,
          role_ids,
          dept_ids,
        };
        Ok(res)
      }
    }
  }

  /// add 添加
  #[oai(path="/system/user/add", method="post")]
  pub async fn add(&self, add_req: Json<AddReq>, user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::add(db, add_req.0, user.id).await;
    match res {
      Ok(x) => Res::with_msg(&x),
      Err(e) => Res::with_err(&e.to_string()),
    }
  }

  /// delete 完全删除
  #[oai(path="/system/user/delete", method="delete")]
  pub async fn delete(&self, delete_req: Json<DeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::delete(db, delete_req.0).await;
    match res {
      Ok(x) => Res::with_msg(&x),
      Err(e) => Res::with_err(&e.to_string()),
    }
  }

  // edit 修改
  #[oai(path="/system/user/edit", method="put")]
  pub async fn edit(&self, edit_req: Json<EditReq>, user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::edit(db, edit_req.0, user.id).await;
    match res {
      Ok(x) => Res::with_msg(&x),
      Err(e) => Res::with_err(&e.to_string()),
    }
  }

  #[oai(path="/system/user/update_profile", method="put")]
  pub async fn update_profile(&self, req: Json<UpdateProfileReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::update_profile(db, req.0).await;
    match res {
      Ok(x) => Res::with_msg(&x),
      Err(e) => Res::with_err(&e.to_string()),
    }
  }

  /// 获取用户登录信息
  #[oai(path="/system/user/get_info", method="get")]
  pub async fn get_info(&self, user: Claims) -> Res<UserInfo> {
    let db = DB.get_or_init(db_conn).await;

    let (role_ids_r, dept_ids_r, user_r) = join!(
        service::sys_user_role::get_role_ids_by_user_id(db, &user.id),
        service::sys_user_dept::get_dept_ids_by_user_id(db, &user.id),
        Self::get_user_info_permission(&user.id),
    );

    let roles = match role_ids_r {
      Ok(x) => x,
      Err(e) => return Res::with_err(&e.to_string()),
    };
    let depts = match dept_ids_r {
      Ok(x) => x,
      Err(e) => return Res::with_err(&e.to_string()),
    };
    let (user, permissions) = match user_r {
      Ok((x, y)) => (x, y),
      Err(e) => return Res::with_err(&e.to_string()),
    };

    let res = UserInfo { user, roles, depts, permissions };

    Res::with_data(res)
  }

  // 获取用户信息以及权限
  async fn get_user_info_permission(user_id: &str) -> Result<(UserWithDept, Vec<String>)> {
    let db = DB.get_or_init(db_conn).await;
    //  获取用户信息
    let user_info = service::sys_user::get_by_id(db, user_id).await?;

    // 检查是否超管用户
    let permissions = if CFG.system.super_user.contains(&user_id.to_string()) {
      vec!["*:*:*".to_string()]
    } else {
      let (apis, _) = service::sys_menu::get_role_permissions(db, &user_info.user.role_id).await?;
      apis
    };
    Ok((user_info, permissions))
  }

  // edit 修改
  #[oai(path="/system/user/reset_passwd", method="put")]
  pub async fn reset_passwd(&self, req: Json<ResetPwdReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::reset_passwd(db, req.0).await;
    match res {
      Ok(x) => Res::with_msg(&x),
      Err(e) => Res::with_err(&e.to_string()),
    }
  }

  #[oai(path="/system/user/update_passwd", method="put")]
  pub async fn update_passwd(&self, req: Json<UpdatePwdReq>, user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::update_passwd(db, req.0, &user.id).await;
    match res {
      Ok(x) => Res::with_msg(&x),
      Err(e) => Res::with_err(&e.to_string()),
    }
  }

  // edit 修改
  #[oai(path="/system/user/change_status", method="put")]
  pub async fn change_status(&self, req: Json<ChangeStatusReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::change_status(db, req.0).await;
    match res {
      Ok(x) => Res::with_msg(&x),
      Err(e) => Res::with_err(&e.to_string()),
    }
  }
  // fresh_token 刷新token
  #[oai(path="/system/user/fresh_token", method="put")]
  pub async fn fresh_token(&self, user: Claims) -> Res<AuthBody> {
    let res = service::sys_user::fresh_token(user).await;
    match res {
      Ok(x) => Res::with_data(x),
      Err(e) => Res::with_err(&e.to_string()),
    }
  }

  #[oai(path="/system/user/change_role", method="put")]
  pub async fn change_role(&self, req: Json<ChangeRoleReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::change_role(db, req.0).await;
    match res {
      Ok(x) => Res::with_msg(&x),
      Err(e) => Res::with_err(&e.to_string()),
    }
  }
  #[oai(path="/system/user/change_dept", method="put")]
  pub async fn change_dept(&self, req: Json<ChangeDeptReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::change_dept(db, req.0).await;
    match res {
      Ok(x) => Res::with_msg(&x),
      Err(e) => Res::with_err(&e.to_string()),
    }
  }

  #[oai(path="/system/user/update_avatar", method="post")]
  pub async fn update_avatar(&self, multipart: Multipart, user: Claims) -> Res<String> {
    let res = service::common::upload_file(multipart).await;
    match res {
      Ok(x) => {
        let db = DB.get_or_init(db_conn).await;
        let res = service::sys_user::update_avatar(db, &x, &user.id).await;
        match res {
          Ok(y) => Res::with_data_msg(x, &y),
          Err(e) => Res::with_err(&e.to_string()),
        }
      }
      Err(e) => Res::with_err(&e.to_string()),
    }
  }

}

