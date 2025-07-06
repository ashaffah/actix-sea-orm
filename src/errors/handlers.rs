use actix_web::{ web, HttpResponse, Responder };
use tera::Context;

use crate::configs::app::AppsConfig;

pub async fn not_found(ctx: web::Data<AppsConfig>) -> impl Responder {
    let context = Context::new();

    let rendered = ctx.templates
        .render("404.html.tera", &context)
        .unwrap_or_else(|_| "<h1>404 - Page Not Found</h1>".to_string());

    HttpResponse::NotFound().content_type("text/html").body(rendered)
}
