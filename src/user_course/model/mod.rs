use diesel::prelude::*;
use crate::schema::*;
use crate::user::model::User;
use crate::class::model::Class;

#[derive(Queryable, Identifiable, Debug, Associations)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Class, foreign_key = class_id))]
#[diesel(table_name = users_classes)]
pub struct UserClass {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub class_id: Option<i32>,
}