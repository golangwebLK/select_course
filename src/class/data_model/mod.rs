use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct Classes {
    pub id: i32,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    pub name: String,
    pub enable: bool,
    pub note: String,
    #[serde(rename = "specialityID")]
    pub speciality_id: i32,
    #[serde(rename = "classTimeStart")]
    pub class_time_start: String,
    #[serde(rename = "classTimeEnd")]
    pub class_time_end: String,
    #[serde(rename = "specialityName")]
    pub speciality_name: String,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Data {
    pub classes: Vec<Classes>,
    pub count: i64,
    pub page: i64,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Root {
    pub code: i64,
    pub data: Data,
    pub msg: String,
}