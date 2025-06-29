use actix_web::middleware::Logger;
use actix_web::{ web, App, HttpServer };
use configs::cors::cors_config;
use configs::env::{ get_server_host, get_server_port };
use errors::handlers::not_found;
use routes::{ public_routes };

pub mod db;
pub mod routes;
pub mod configs;
pub mod errors;
pub mod qr;

use crate::configs::env::init_env;
use crate::configs::app::AppsConfig;
use crate::db::mysql::init_db;

pub async fn run() -> std::io::Result<()> {
    init_env();

    let db = init_db().await;
    let service_config = AppsConfig::new(db);

    HttpServer::new(move || {
        let api_scope = web::scope("/api").configure(public_routes);

        App::new()
            .wrap(Logger::default())
            .wrap(cors_config())
            .app_data(web::Data::new(service_config.clone()))
            .service(api_scope)
            .default_service(web::to(not_found))
    })
        .bind((get_server_host(), get_server_port()))?
        .run().await
}
