use actix_web::{HttpResponse, web};

pub mod model;
pub mod handler;
pub mod data_model;


pub fn router_config_class(cfg: &mut web::ServiceConfig){
    cfg.service(web::resource("/")
        .route(web::get().to(HttpResponse::MethodNotAllowed))
        .route(web::post().to(handler::create_class))
        .route(web::put().to(HttpResponse::MethodNotAllowed))
        .route(web::delete().to(HttpResponse::MethodNotAllowed))
    );
}