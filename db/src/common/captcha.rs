use poem_openapi::Object;
use serde::Serialize;

#[derive(Debug, Serialize, Object)]
pub struct CaptchaImage {
    pub captcha_on_off: bool,
    pub uuid: String,
    pub img: String,
}
