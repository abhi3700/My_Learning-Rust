use actix::SyncArbiter;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

mod actors;
mod db_models;
mod db_utils;
mod messages;
mod schema;
mod services;

use db_utils::{get_pool, AppState, DbActor};
use dotenv::dotenv;
use services::{create_user_article, fetch_user_articles, fetch_users};
use std::env;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
            }))
            .configure(services::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
