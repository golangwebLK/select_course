
use diesel::prelude::*;
use serde::Deserialize;
use crate::schema::*;
#[derive(Insertable, Debug,Deserialize,Queryable)]
#[diesel(table_name = student_data)]
pub struct StudentData {
    pub id: u64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub name: String,
    pub age: i64,
    pub school: Option<String>,
    pub telephone: String,
    pub sex: String,
    pub status: Option<bool>,
    pub note: Option<String>,
    pub specialities: Option<String>,
    #[diesel(column_name = username)]
    pub username: String,
    #[diesel(column_name = password)]
    pub password: String,
}


