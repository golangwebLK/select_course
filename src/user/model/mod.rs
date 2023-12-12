use diesel::prelude::*;
use crate::schema::*;
#[derive(Queryable, Identifiable,Insertable, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub student_id: Option<i32>,
}