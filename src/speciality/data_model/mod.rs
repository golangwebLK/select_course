use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Struct {
    pub id: i32,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
    pub name: Option<String>,
    pub enable: bool,
    pub note: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub count: i64,
    pub page: i64,
    pub specialities: Vec<Struct>,
}

#[derive(Serialize, Deserialize)]
pub struct Root {
    pub data: Data,
    pub msg: String,
}