use crate::{
    db_utils::{AppState, DbActor},
    messages::CreateArticle,
};
use actix::Addr;
use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};

use crate::messages::{FetchUser, FetchUserArticles};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateArticleBody {
    pub title: String,
    pub content: String,
}

#[get("/users")]
pub async fn fetch_users(state: Data<AppState>) -> impl Responder {
    // "GET /users".to_string()

    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(FetchUser).await {
        Ok(Ok(users)) => HttpResponse::Ok().json(users),
        Ok(Err(_)) => HttpResponse::InternalServerError().json("No users found"),
        // Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
        _ => HttpResponse::InternalServerError().json("Unable to fetch users"),
    }
}

#[get("/users/{id}/articles")]
pub async fn fetch_user_articles(path: web::Path<i32>, state: Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    // "GET /users/{id}/articles".to_string()

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUserArticles { user_id: id }).await {
        Ok(Ok(articles)) => HttpResponse::Ok().json(articles),
        Ok(Err(_)) => HttpResponse::NotFound().json(format!("No articles found for user {id}")),
        // Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
        _ => HttpResponse::InternalServerError().json("Unable to fetch user articles"),
    }
}

#[post("/users/{id}/articles")]
pub async fn create_user_article(
    path: web::Path<i32>,
    state: Data<AppState>,
    body: web::Json<CreateArticleBody>,
) -> impl Responder {
    // "POST /users".to_string()
    let id = path.into_inner();

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db
        .send(CreateArticle {
            title: body.title.to_string(),
            content: body.content.to_string(),
            created_by: id,
        })
        .await
    {
        Ok(Ok(article)) => HttpResponse::Ok().json(article),
        Ok(Err(_)) => HttpResponse::InternalServerError().json("Unable to create article"),
        // Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
        _ => HttpResponse::InternalServerError()
            .json("Unable to create article due to internal server error"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(fetch_users)
        .service(create_user_article)
        .service(fetch_user_articles);
}
