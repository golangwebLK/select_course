// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Response;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Response = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub code: i64,

    pub data: Data,

    pub msg: String,
}

#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub count: i64,

    pub page: i64,

    pub student_data: Vec<StudentDatum>,
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct StudentDatum {
    pub age: Option<i32>,

    pub created_at: Option<String>,

    pub deleted_at: Option<serde_json::Value>,

    pub id: Option<i32>,

    pub name: Option<String>,

    pub school: Option<String>,

    pub sex: Option<String>,

    pub status: Option<bool>,

    pub telephone: Option<String>,

    pub updated_at: Option<String>,
}