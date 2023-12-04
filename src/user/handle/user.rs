use actix_web::{HttpResponse, Responder, web};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Deserialize;

use crate::ConnPool;
use crate::schema::student_data::dsl::student_data;
use crate::schema::student_data::password;
use crate::schema::student_data::username;
use crate::user::model::user::StudentData;


#[derive(Debug, Deserialize)]
pub struct LoginRequest{
    username: String,
    password: String,
}

pub async fn create_user(
    student_request: web::Json<StudentData>,
    pool: web::Data<ConnPool>
) -> impl Responder{
    let mut conn = pool.get().unwrap();
    match diesel::insert_into(student_data)
        .values(student_request.into_inner())
        .execute(&mut conn)
    {
        Ok(val) => {
            println!("Successfully inserted new student to database! {}", val);
        }
        Err(e) => {
            println!("Failed to insert new student {}!", e.to_string());
        }
    };
    HttpResponse::Ok().finish()
}

pub async fn login(
    login_request: web::Json<LoginRequest>,
    pool: web::Data<ConnPool>
) -> impl Responder{
    let login = login_request.into_inner();
    let mut conn = pool.get().unwrap();
    return match student_data
        .filter(username.eq(login.username))
        .select(password).first::<String>(&mut conn) {
        Ok(p) => {
            if p == login.password {
                return HttpResponse::Ok().body("login succeed");
            }
            HttpResponse::Ok().body("login fail")
        }
        Err(_) => {
            HttpResponse::Ok().body("login fail")
        }
    };
}
