pub mod search;

use std::collections::{HashSet};
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use common::util::response::ApiResponse;
use crate::{ ConnPoolSqlx};
use crate::user::handle::user::ClassSearch;
use crate::user_course::model::UserClass;

#[derive(Debug, Deserialize)]
pub struct SelectRequest{
    pub user_id: Option<i32>,
    pub class_id: Option<i32>,
}
pub async fn select_course(
    select_request: web::Json<SelectRequest>,
    pool: web::Data<ConnPoolSqlx>
) -> impl Responder{
    let conn = pool.get_ref();
    let select = select_request.into_inner();

    let result = sqlx::query_as::<_,UserClass>(
        r#"
        SELECT *
        FROM users_classes
        "#,
    )
        .fetch_all(conn)
        .await;
    let mut user_ids = HashSet::new();

    if let Ok(user_classes) = result {
        if user_classes.len() > 40 {
            return HttpResponse::Ok().json(ApiResponse {
                code: 1,
                msg: "班级人数已满".to_string(),
                data: "",
            });
        }

        for user_class in user_classes {
            let user_id = user_class.user_id.unwrap();
            user_ids.insert(user_id);
        }
    }

    match user_ids.get(&select.user_id.unwrap()) {
        Some(_user_classes) => {
            return HttpResponse::Ok()
                .json(ApiResponse{
                    code: 0,
                    msg: "select course success".to_string(),
                    data: (),
                });
        }
        None =>{
            let new_user_class = UserClass {
                id: None,
                user_id: select.user_id,
                class_id: select.class_id,
            };

            let _inserted_user_class = sqlx::query(
                r#"
                    INSERT INTO users_classes (user_id, class_id)
                    VALUES (?, ?)
                    RETURNING id, user_id, class_id
                    "#,
            )
                .bind(&new_user_class.user_id)
                .bind(&new_user_class.class_id)
                .execute(conn)
                .await
                .expect("Error inserting new user class");

            let cls = sqlx::query_as!(ClassSearch,
            "SELECT * FROM classes WHERE class_id = ?",
            new_user_class.class_id
            )
                .fetch_one(conn)
                .await
                .expect("Error loading data");
            HttpResponse::Ok()
                .json(ApiResponse{
                    code: 0,
                    msg: "select course success".to_string(),
                    data: cls,
                })
        }
    }
}