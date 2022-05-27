use app_derive::QueryObject;
use poem_openapi::Object;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Object, QueryObject)]
pub struct CaptchaImage {
    pub captcha_on_off: bool,
    pub uuid: String,
    pub img: String,
}
