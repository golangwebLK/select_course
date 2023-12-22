use actix_web::{HttpResponse, web};

pub mod data_model;
pub mod handle;
pub mod model;

pub fn router_config_speciality(cfg: &mut web::ServiceConfig){
    cfg.service(web::resource("/")
        .route(web::get().to(HttpResponse::MethodNotAllowed))
        .route(web::post().to(handle::create_speciality))
        .route(web::put().to(HttpResponse::MethodNotAllowed))
        .route(web::delete().to(HttpResponse::MethodNotAllowed))
    );
}