use actix_web::web;
use crate::modules::qr::handlers::{ generate_qr, generate_qrs, get_svg };

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(generate_qr).service(generate_qrs).service(get_svg);
}
