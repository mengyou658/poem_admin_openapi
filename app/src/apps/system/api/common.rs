use db::{common::{captcha::CaptchaImage, res::Res}, DB, db_conn, system::models::server_info::SysInfo};
use poem::{handler, Request};
use poem::web::Json;
use db::system::models::sys_user::UserLoginReq;
use crate::apps::system::service;
use crate::utils::jwt::AuthBody;
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService};

use super::super::service::server_info::{get_oper_sys_info, SYSINFO};

/// 无需授权Api.通用模块
pub struct CommonApi;

#[OpenApi]
impl CommonApi {

  /// 获取验证码图片
  #[oai(path="/system/comm/get_captcha", method="get")]
  pub async fn get_captcha(&self) -> Res<CaptchaImage> {
    let res = super::super::service::common::get_captcha();
    Res::with_data(res)
  }

  /// 获取系统信息
  #[oai(path="/system/server/get_server_info", method="get")]
  pub async fn get_server_info(&self) -> Res<SysInfo> {
    let sys_info = SYSINFO.lock().await;
    let info = match &*sys_info {
      Some(sys_info) => sys_info.clone(),
      None => {
        let res = get_oper_sys_info().await;
        res
      }
    };
    Res::with_data(info)
  }

  /// 用户登录
  #[oai(path="/system/comm/login", method="post")]
  pub async fn login(&self, login_req: Json<UserLoginReq>, request: &Request) -> Res<AuthBody> {
    let db = DB.get_or_init(db_conn).await;
    let res = match service::sys_user::login(db, login_req.0, request).await {
      Ok(x) => Res::with_data(x),
      Err(e) => Res::with_err(&e.to_string()),
    };

    res
  }

}
