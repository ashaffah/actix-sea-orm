use actix_web::web::{ self, scope };

use crate::{ qr };

pub fn public_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/public").service(scope("/v1").configure(qr::routes::config)));
}
