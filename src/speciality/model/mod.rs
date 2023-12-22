use diesel::prelude::*;
use serde::Serialize;
use crate::schema::*;
#[derive(Queryable, Identifiable,Insertable, Debug,Serialize)]
#[diesel(table_name = specialities)]
pub struct Speciality {
    pub id: Option<i32>,
    pub name: String,
    pub enable: bool,
    pub note: Option<String>,
    pub speciality_id: Option<i32>,
}