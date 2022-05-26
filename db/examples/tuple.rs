use poem_openapi::Object;
use serde::{Deserialize};
use app_derive::QueryObject;
#[derive(Deserialize, Object, QueryObject)]
pub struct Person {
    name: String,
    age: usize
}

fn main() {

    let str =  "name=baguette&age=1";

    let res: Person = serde_qs::from_str(str).unwrap();

}
