extern crate openssl;
extern crate diesel;
mod user;
mod schema;
mod class;
mod user_course;

use actix_web::{App, HttpResponse, HttpServer, middleware, web};
use diesel::r2d2::{ConnectionManager};
use diesel::{MysqlConnection, r2d2};
use dotenv::dotenv;
use common::middleware::auth;

pub type ConnPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    openssl_probe::init_ssl_cert_env_vars();
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    let database_url: String = std::env::var("DATABASE_URL").unwrap();
    let server_ip: String = std::env::var("SERVER_IP").unwrap();

    let pool: r2d2::Pool<ConnectionManager<MysqlConnection>> = ConnPool::builder()
        .max_size(5)
        .build(ConnectionManager::<MysqlConnection>::new(database_url)).unwrap();

    HttpServer::new(move|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().add(("Version", "0.0.1")))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default().log_target("request::log"))
            .service(web::resource("/api/v1/login")
                .route(web::post().to(user::handle::user::login))
            )
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api/v1").configure(router_config)
            )
            .default_service(web::route().to(HttpResponse::NotFound))

    })
        .bind(server_ip)?
        // .workers(1)
        .run()
        .await
}



fn router_config(cfg: &mut web::ServiceConfig) {
    cfg .service(web::scope("/user")
        .wrap(auth::Auth)
        .configure(user::router_config_user));
}