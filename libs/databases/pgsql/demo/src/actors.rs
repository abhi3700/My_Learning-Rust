use crate::db_models::{Article, User};
use crate::db_utils::DbActor;
use crate::messages::{FetchUser, FetchUserArticles};
use crate::schema::articles::{dsl::*, id as article_id};
use crate::schema::users::dsl::*;
use actix::Handler;
use diesel::{self, prelude::*};

impl Handler<FetchUser> for DbActor {
    type Result = QueryResult<Vec<User>>;

    fn handle(&mut self, _msg: FetchUser, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Fetch User: Unable to establish connection");

        users.get_results::<User>(&mut conn)
    }
}

impl Handler<FetchUserArticles> for DbActor {
    type Result = QueryResult<Vec<Article>>;

    fn handle(&mut self, msg: FetchUserArticles, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Fetch User Articles: Unable to establish connection");

        articles
            .filter(created_by.eq(msg.user_id))
            .get_results::<Article>(&mut conn)
    }
}
