mod auth;
mod config;
mod db;
mod middleware;
mod models;
mod services;
mod utils;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let config = config::Config::from_env().expect("Failed to load configuration");
    let pool = db::create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");
    let jwt_util = utils::jwt::JwtUtil::new(&config.jwt_secret);
    let host = config.host.clone();
    let port = config.port;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(jwt_util.clone()))
            .app_data(web::Data::new(config.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(middleware::auth_middleware::AuthMiddleware::new(
                jwt_util.clone(),
            ))
            .configure(services::user_service::configure)
            .configure(services::post_service::configure)
            .route("/health", web::get().to(|| async { "OK" }))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
