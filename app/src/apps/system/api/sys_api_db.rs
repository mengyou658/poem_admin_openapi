use db::{
    common::res::Res,
    db_conn,
    system::{
        entities::sys_api_db,
        models::sys_api_db::{AddEditReq, SearchReq},
    },
    DB,
};
use poem::{
    handler,
    web::{Json},
};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService};

use super::super::service;

pub struct SysApiDB;

#[OpenApi]
impl SysApiDB {


  /// add 添加
  #[oai(path="/api_db/add", method="post")]
  pub async fn add(&self, req: Json<AddEditReq>) -> Res<String> {
      let db = DB.get_or_init(db_conn).await;
      let res = service::sys_api_db::add(db, req.0).await;
      match res {
          Ok(x) => Res::with_msg(&x),
          Err(e) => Res::with_err(&e.to_string()),
      }
  }

  /// 按id获取
  /// db 数据库连接
  #[oai(path="/api_db/get_by_id", method="get")]
  pub async fn get_by_id(&self, req: Query<Option<SearchReq>>) -> Res<Vec<sys_api_db::Model>> {
      let db = DB.get_or_init(db_conn).await;
      let res = service::sys_api_db::get_by_id(db, &req.0.unwrap().api_id).await;
      match res {
          Ok(x) => Res::with_data(x),
          Err(e) => Res::with_err(&e.to_string()),
      }
  }


}
