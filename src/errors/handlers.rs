use actix_web::{ HttpResponse, Responder, Result };

pub async fn not_found() -> Result<impl Responder> {
    Ok(
        HttpResponse::NotFound()
            .content_type("application/json")
            .json(r#"{"error": "Route not found"}"#)
    )
}
