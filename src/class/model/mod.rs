use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use crate::schema::*;
#[derive(Queryable, Identifiable,Insertable, Debug,Serialize)]
#[diesel(table_name = classes)]
pub struct Class {
    pub id: Option<i32>,
    pub class_name: String,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub note: Option<String>,
}