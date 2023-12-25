
extern crate diesel;
mod user;

mod speciality;
mod schema;
mod class;
mod user_course;

use actix_web::{App, HttpResponse, HttpServer, middleware, web};
use diesel::r2d2::{ConnectionManager};
use diesel::{MysqlConnection, r2d2};
use dotenv::dotenv;
use sqlx::{MySql, Pool};
use sqlx::mysql::MySqlPoolOptions;
use common::middleware::auth;

pub type ConnPoolDiesel = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub type ConnPoolSqlx = Pool<MySql>;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    let database_url: String = std::env::var("DATABASE_URL").unwrap();
    let server_ip: String = std::env::var("SERVER_IP").unwrap();

    let pool_diesel: r2d2::Pool<ConnectionManager<MysqlConnection>> = ConnPoolDiesel::builder()
        .max_size(5)
        .build(ConnectionManager::<MysqlConnection>::new(&database_url)).unwrap();

    let pool_sqlx = MySqlPoolOptions::new()
        .max_connections(20)
        .connect(&database_url).await.unwrap();

    HttpServer::new(move|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().add(("Version", "0.0.1")))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default().log_target("request::log"))
            .service(web::resource("/api/v1/login")
                .route(web::post().to(user::handle::user::login))
            )
            .app_data(web::Data::new(pool_diesel.clone()))
            .app_data(web::Data::new(pool_sqlx.clone()))
            .service(
                web::scope("/api/v1")
                    .wrap(auth::Auth)
                    .configure(router_config)
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
            .configure(user::router_config_user))
        .service(web::scope("/class")
            .configure(class::router_config_class))
        .service(web::scope("/speciality")
            .configure(speciality::router_config_speciality))
        .service(web::scope("/user_course")
            .configure(user_course::router_config_user_course));
}