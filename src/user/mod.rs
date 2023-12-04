pub mod handle;
pub mod model;

use actix_web::{web, HttpResponse};

pub fn router_config_user(cfg: &mut web::ServiceConfig){
    cfg.service(web::resource("/")
        .route(web::get().to(HttpResponse::MethodNotAllowed))
        .route(web::post().to(handle::user::create_user))
        .route(web::put().to(HttpResponse::MethodNotAllowed))
        .route(web::delete().to(HttpResponse::MethodNotAllowed))
    )
        .service(web::resource("/login")
        .route(web::post().to(handle::user::login))
    );
}