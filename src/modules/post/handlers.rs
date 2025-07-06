use actix_web::{ error, get, post, web, Error, HttpRequest, HttpResponse };
use entity::{
    posts::{ Column as PostsColumn, Model as PostsModel, ActiveModel as PostsActiveModel },
    prelude::Posts,
};
use sea_orm::{
    ActiveValue::Set,
    ColumnTrait,
    Condition,
    EntityTrait,
    PaginatorTrait,
    QueryFilter,
    QuerySelect,
};
use serde::Deserialize;

use crate::configs::app::AppsConfig;

const DEFAULT_PER_PAGE: u64 = 10;

#[derive(Debug, Deserialize)]
pub struct Params {
    page: Option<u64>,
    per_page: Option<u64>,
    search: Option<String>,
}

#[get("/")]
async fn list_posts(req: HttpRequest, data: web::Data<AppsConfig>) -> Result<HttpResponse, Error> {
    let template = &data.templates;
    let conn = &data.db;

    let params = web::Query::<Params>::from_query(req.query_string()).unwrap();

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(DEFAULT_PER_PAGE);
    let search = params.search.clone().unwrap_or_default();

    let limit = per_page;
    let offset = (page - 1) * per_page;

    let mut query = Posts::find();

    if !search.is_empty() {
        query = query.filter(
            Condition::any()
                .add(PostsColumn::Title.contains(&search))
                .add(PostsColumn::Text.contains(&search))
        );
    }

    let num_pages = match query.clone().count(conn).await {
        Ok(count) => count,
        Err(e) => {
            return Ok(
                HttpResponse::InternalServerError().json(format!("Failed to count users: {}", e))
            );
        }
    };

    let posts = match query.limit(limit).offset(offset).all(conn).await {
        Ok(users) => users,
        Err(e) => {
            return Ok(
                HttpResponse::InternalServerError().json(format!("Failed to get users: {}", e))
            );
        }
    };

    let mut ctx = tera::Context::new();
    ctx.insert("posts", &posts);
    ctx.insert("page", &page);
    ctx.insert("per_page", &per_page);
    ctx.insert("num_pages", &num_pages);

    let body = template
        .render("index.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/new")]
async fn new(data: web::Data<AppsConfig>) -> Result<HttpResponse, Error> {
    let template = &data.templates;
    let ctx = tera::Context::new();
    let body = template
        .render("new.html.tera", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/")]
async fn create(
    data: web::Data<AppsConfig>,
    post_form: web::Form<PostsModel>
) -> Result<HttpResponse, Error> {
    let conn = &data.db;
    let template = &data.templates;

    let form: PostsModel = post_form.into_inner().into();

    let new_post = PostsActiveModel {
        title: Set(form.title.to_owned()),
        text: Set(form.text.to_owned()),
        ..Default::default()
    };

    match Posts::insert(new_post).exec(conn).await {
        Ok(_) => Ok(HttpResponse::Found().append_header(("location", "/")).finish()),
        Err(_) => {
            let ctx = tera::Context::new();
            let body = template
                .render("500.html.tera", &ctx)
                .unwrap_or_else(|_| "<h1>500 - Internal Server Error</h1>".to_string());
            Ok(HttpResponse::InternalServerError().content_type("text/html").body(body))
        }
    }
}

#[get(r#"/{id:\d+}"#)]
async fn edit(data: web::Data<AppsConfig>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = &data.db;
    let template = &data.templates;
    let id = id.into_inner();

    let post: Option<PostsModel> = Posts::find_by_id(id)
        .one(conn).await
        .map_err(|_| error::ErrorInternalServerError("Database query error"))?;

    let mut ctx = tera::Context::new();
    let body = match post {
        Some(post) => {
            ctx.insert("post", &post);

            template
                .render("edit.html.tera", &ctx)
                .map_err(|_| error::ErrorInternalServerError("Template error"))
        }
        None => {
            ctx.insert("uri", &format!("/{}", id));

            template
                .render("404.html.tera", &ctx)
                .map_err(|_| error::ErrorInternalServerError("Template error"))
        }
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(body?))
}

#[post("/{id}")]
async fn update(
    data: web::Data<AppsConfig>,
    id: web::Path<i32>,
    post_form: web::Form<PostsModel>
) -> Result<HttpResponse, Error> {
    let conn = &data.db;
    let id = id.into_inner();
    let form = post_form.into_inner();

    match Posts::find_by_id(id).one(conn).await {
        Ok(Some(post)) => {
            let mut post: PostsActiveModel = post.into();

            post.title = Set(form.title.to_owned());
            post.text = Set(form.text.to_owned());
            match Posts::update(post).exec(conn).await {
                Ok(_) => Ok(HttpResponse::Found().append_header(("location", "/")).finish()),
                Err(e) =>
                    Ok(
                        HttpResponse::InternalServerError().json(
                            format!("Failed to update user: {}", e)
                        )
                    ),
            }
        }
        Ok(None) => Err(error::ErrorNotFound("Post not found")),
        Err(_) => Err(error::ErrorInternalServerError("Database error")),
    }
}

#[post("/delete/{id}")]
async fn delete(data: web::Data<AppsConfig>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = &data.db;
    let id = id.into_inner();

    match Posts::find_by_id(id).one(conn).await {
        Ok(Some(post)) => {
            let post: PostsActiveModel = post.into();
            match Posts::delete(post).exec(conn).await {
                Ok(_) => Ok(HttpResponse::Found().append_header(("location", "/")).finish()),
                Err(e) =>
                    Ok(
                        HttpResponse::InternalServerError().json(
                            format!("Failed to delete user: {}", e)
                        )
                    ),
            }
        }
        Ok(None) => Err(error::ErrorNotFound("Post not found")),
        Err(_) => Err(error::ErrorInternalServerError("Database error")),
    }
}
