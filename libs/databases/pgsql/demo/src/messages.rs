//! Define messages for actix actors

use crate::db_models::{Article, User};
use actix::Message;
use diesel::QueryResult;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct FetchUser;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Article>>")]
pub struct FetchUserArticles {
    pub user_id: u32,
}
