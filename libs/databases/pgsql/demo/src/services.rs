use actix_web::{get, post, web, Responder};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateArticleBody {
    pub title: String,
    pub body: String,
}

#[get("/users")]
pub async fn fetch_users() -> impl Responder {
    "GET /users".to_string()
}

#[post("/users/{id}/articles")]
pub async fn create_user_article(
    id: web::Path<u32>,
    body: web::Json<CreateArticleBody>,
) -> impl Responder {
    "POST /users".to_string()
}

#[get("/users/{id}/articles")]
pub async fn fetch_user_articles(id: web::Path<u32>) -> impl Responder {
    "GET /users/{id}/articles".to_string()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(fetch_users)
        .service(create_user_article)
        .service(fetch_user_articles);
}
