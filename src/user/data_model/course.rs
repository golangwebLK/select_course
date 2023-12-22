use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Debug)]
pub struct Struct {
    pub id: Option<i32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
    #[serde(rename = "classID")]
    pub class_id: Option<i32>,
    #[serde(rename = "studentDataID")]
    pub student_data_id: Option<i32>,
    #[serde(rename = "totalCourses")]
    pub total_courses: Option<i32>,
    #[serde(rename = "notTakingCourses")]
    pub not_taking_courses: Option<i32>,
    #[serde(rename = "takenCourses")]
    pub taken_courses: Option<i32>,
    #[serde(rename = "studentName")]
    pub student_name: Option<String>,
    #[serde(rename = "className")]
    pub class_name: Option<String>,
    pub note: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub count: i64,
    pub course: Vec<Struct>,
    pub page: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Root {
    pub code: i64,
    pub data: Data,
    pub msg: String,
}