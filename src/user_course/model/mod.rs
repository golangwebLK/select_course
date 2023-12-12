use diesel::prelude::*;
use crate::schema::*;
use crate::user::model::User;
use crate::course::model::Course;

#[derive(Queryable, Identifiable, Debug, Associations)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Course, foreign_key = course_id))]
#[diesel(table_name = users_courses)]
pub struct UserCourse {
    pub id: i32,
    pub user_id: Option<i32>,
    pub course_id: Option<i32>,
}