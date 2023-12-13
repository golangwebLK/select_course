use diesel::prelude::*;
use crate::schema::*;
#[derive(Queryable, Identifiable, Debug)]
#[diesel(table_name = classes)]
pub struct Class {
    pub id: Option<i32>,
    pub class_name: String,
    pub start_time: Option<chrono::NaiveDateTime>,
    pub end_time: Option<chrono::NaiveDateTime>,
    pub note: Option<String>,
}