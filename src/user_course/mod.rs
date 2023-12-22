use actix_web::{HttpResponse, web};

mod model;
mod handle;


pub fn router_config_user_course(cfg: &mut web::ServiceConfig){
    cfg.service(web::resource("/")
        .route(web::get().to(HttpResponse::MethodNotAllowed))
        .route(web::post().to(handle::select_course))
        .route(web::put().to(HttpResponse::MethodNotAllowed))
        .route(web::delete().to(HttpResponse::MethodNotAllowed))
    );
}