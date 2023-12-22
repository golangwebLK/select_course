use actix_web::{HttpResponse, Responder, web};
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Deserialize;
use common::util::response::ApiResponse;
use crate::ConnPool;
use crate::schema::users_classes;
use crate::user_course::model::UserClass;

#[derive(Debug, Deserialize)]
pub struct SelectRequest{
    pub user_id: Option<i32>,
    pub class_id: Option<i32>,
}
pub async fn select_course(
    select_request: web::Json<SelectRequest>,
    pool: web::Data<ConnPool>
) -> impl Responder{
    let mut conn = pool.get().unwrap();
    let select = select_request.into_inner();
    let mut query = users_classes::dsl::users_classes.into_boxed();

    if let Some(user_id) = select.user_id {
        query = query.filter(users_classes::user_id.is_not_null()
            .and(users_classes::user_id.eq(user_id)));
    }
    let result = query.load::<UserClass>(&mut conn);

    println!("{:?}",result);

    match result {
        Ok(user_classes) => {
            if user_classes.len() != 0{
                return HttpResponse::Ok()
                    .json(ApiResponse{
                        code: 0,
                        msg: "select course success".to_string(),
                        data: (),
                    })
            }
            let new_user_class = UserClass{
                id: None,
                user_id: select.user_id,
                class_id: select.class_id,
            };
            diesel::insert_into(users_classes::dsl::users_classes)
                .values(&new_user_class)
                .execute(&mut conn)
                .expect("Error inserting new users");
            HttpResponse::Ok()
                .json(ApiResponse{
                    code: 0,
                    msg: "select course success".to_string(),
                    data: new_user_class,
                })
        }
        Err(e) => {
            HttpResponse::Ok()
                .json(ApiResponse{
                    code: 0,
                    msg: "select course fail".to_string(),
                    data: e.to_string(),
                })
        }
    }
}