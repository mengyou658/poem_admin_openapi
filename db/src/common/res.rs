use std::fmt::Debug;

use app_derive::QueryObject;
use poem::{http::StatusCode, IntoResponse, Response, Error};
use poem_openapi::{
    payload::Json,
    types::{ParseError, ParseFromJSON, ToJSON},
    Object, OpenApi, OpenApiService, ApiResponse, error::ParseRequestPayloadError,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Serialize, Object)]
/// 查 数据返回
pub struct ListData<T>
where
    T: ParseFromJSON + ToJSON + Send + Sync,{
    pub list: Vec<T>,
    pub total: usize,
    pub total_pages: usize,
    pub page_num: usize,
}
/// 分页参数
#[derive(Deserialize, Clone, Debug, Serialize, Default, Object, QueryObject)]
pub struct PageParams {
    pub page_num: Option<usize>,
    pub page_size: Option<usize>,
    pub sort: Option<String>,
}

/// 数据统一返回格式
#[derive(Debug, Serialize, Default, Object)]
pub struct ResObj<T>
where
    T: ParseFromJSON + ToJSON + Send + Sync,
{
    pub code: Option<i32>,
    pub data: Option<T>,
    pub msg: Option<String>,
}

/// 填入到extensions中的数据
#[derive(Debug)]
pub struct ResJsonString(pub String);


// #[allow(unconditional_recursion)]
// impl<T> IntoResponse for ResObj<T>
// where
//     T: Serialize + Send + Sync + Debug + ParseFromJSON + ToJSON + 'static,
// {
//     fn into_response(self) -> Response {
//         let data = Self {
//             code: self.code,
//             data: self.data,
//             msg: self.msg,
//         };
//         let json_string = match serde_json::to_string(&data) {
//             Ok(v) => v,
//             Err(e) => {
//                 return Response::from((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
//             }
//         };
//         let res_json_string = ResJsonString(json_string.clone());
//         let mut response = json_string.into_response();
//         response.extensions_mut().insert(res_json_string);
//         response
//     }
// }


#[derive(ApiResponse)]
#[oai(bad_request_handler = "my_bad_request_handler")]
pub enum Res<T: ParseFromJSON + ToJSON + Send + Sync> {
    #[oai(status = 200)]
    Ok(Json<ResObj<T>>),
    #[oai(status = 500)]
    Err(Json<ResObj<T>>),

}

fn my_bad_request_handler<T: ParseFromJSON + ToJSON + Send + Sync>(err: Error) -> Res<T> {
    if err.is::<ParseRequestPayloadError>() {
        Res::Ok(Json(ResObj {
            code: Some(200),
            msg: Some(err.to_string()),
            data: None,
        }))
    } else {
        Res::Err(Json(ResObj {
            code: Some(500),
            msg: Some(err.to_string()),
            data: None,
        }))
    }
}


impl<T> Res<T>
  where
    T: ParseFromJSON + ToJSON + Send + Sync,
{
  pub fn with_data(data: T) -> Self {
    Res::Ok(Json(ResObj {
      code: Some(200),
      msg: Some("success".to_string()),
      data: Some(data),
    }))
  }
  pub fn with_err(err: &str) -> Self {
    Res::Ok(Json(ResObj {
      code: Some(500),
      msg: Some(err.to_string()),
      data: None,
    }))
  }
  pub fn with_msg(msg: &str) -> Self {
    Res::Ok(Json(ResObj {
      code: Some(200),
      msg: Some(msg.to_string()),
      data: None,
    }))
  }
  #[allow(dead_code)]
  pub fn with_data_msg(data: T, msg: &str) -> Self {
    Res::Ok(Json(ResObj {
      code: Some(200),
      msg: Some(msg.to_string()),
      data: Some(data),
    }))
  }
}
