use diesel::prelude::*;
use crate::schema::*;
#[derive(Queryable, Identifiable, Debug)]
#[diesel(table_name = courses)]
pub struct Course {
    pub id: i32,
    pub course_name: String,
    pub start_time: Option<chrono::NaiveDateTime>,
    pub end_time: Option<chrono::NaiveDateTime>,
    pub note: Option<String>,
}