use actix_cors::Cors;
use actix_web::http;

pub fn cors_config() -> Cors {
    Cors::default()
        .allowed_origin_fn(|origin, _req_head| {
            matches!(origin.to_str(), Ok("http://localhost:3000") | Ok("http://127.0.0.1:3000"))
        })
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
        .supports_credentials()
        .max_age(3600)
}
