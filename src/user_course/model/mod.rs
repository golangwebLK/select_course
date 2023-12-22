use diesel::prelude::*;
use serde::Serialize;
use crate::schema::*;

#[derive(Queryable, Identifiable,Insertable, Debug,Serialize)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Class, foreign_key = class_id))]
#[diesel(table_name = users_classes)]
pub struct UserClass {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub class_id: Option<i32>,
}