use actix_web::{HttpResponse, Responder, web};
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, TextExpressionMethods};
use serde::Deserialize;
use common::util::response::ApiResponse;
use crate::{ConnPoolDiesel};
use crate::schema::{classes, users, users_classes};


#[derive(Deserialize)]
pub struct Request {
    name: String,
    class_name: String,
}
pub async fn search_select_course(
    request: web::Query<Request>,
    pool: web::Data<ConnPoolDiesel>
) -> impl Responder{
    let mut conn = pool.get().unwrap();
    let r = request.into_inner();

    let mut query = users_classes::dsl::users_classes
        .inner_join(users::dsl::users.on(users_classes::user_id.eq(users::student_id)))
        .inner_join(classes::dsl::classes.on(users_classes::class_id.eq(classes::class_id)))
        .into_boxed();

    if !r.name.is_empty() {
        let pattern = format!("%{}%", r.name);
        query = query.filter(users::name.like(pattern));
    }

    if !r.class_name.is_empty() {
        let pattern = format!("%{}%", r.class_name);
        query = query.filter(classes::class_name.like(pattern));
    }

    let rs = query
        .select((users::name, classes::class_name))
        .load::<(Option<String>, String)>(&mut conn)
        .expect("Error loading data");

    HttpResponse::Ok()
        .json(ApiResponse{
            code: 0,
            msg: "success".to_string(),
            data: rs,
        })
}