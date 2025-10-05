use crate::database::DatabaseError;
use crate::model::{Account, Post, Thread, TypedId};
use crate::CommonState;
use axum::extract::{Path, Query, State};
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tracing::instrument;
use crate::config::load;

pub struct PostDto {
    pub post_id: TypedId<Post>,
    pub author: Account,
    pub created: DateTime<Utc>,
    pub body: Arc<String>,
}

pub struct ThreadPage {
    pub thread_id: TypedId<Thread>,
    pub title: String,
    pub page_number: u32,
    pub max_pages: u32,
    pub posts: Vec<PostDto>,
}

pub fn build_router() -> Router<Arc<CommonState>> {
    Router::new().route("/{hru}", get(display_thread_page))
}

#[instrument(skip(state))]
async fn display_thread_page(
    Path(hru): Path<String>,
    Query(offset): Query<Option<i32>>,
    Query(limit): Query<Option<i32>>,
    State(state): State<Arc<CommonState>>,
) -> Html<String> {
    let page = load_thread_page(&state.connection_pool, hru, offset.unwrap_or(0), limit.unwrap_or(10)).await;

    unimplemented!()
}

async fn load_thread_page(
    db_pool: &Pool<Postgres>,
    hru: String,
    offset: i32,
    limit: i32,
) -> Result<ThreadPage, DatabaseError> {
    let result = sqlx::query(r#"SELECT p.id as post_id, t.id as thread_id, a.id as author_id, a.name as author_name, p.created, p.body
FROM dwelling.posts p
         LEFT JOIN dwelling.accounts a on a.id = p.author_id
         LEFT JOIN dwelling.threads t on t.id = p.thread_id
WHERE t.hru = :hru
ORDER BY p.created
OFFSET :offset LIMIT :limit"#).bind(hru).bind(offset).bind(limit).fetch(db_pool);



    unimplemented!()
}
