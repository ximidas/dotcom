use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::Row;

#[derive(Deserialize, PostgresMapper, Serialize, Debug)]
#[pg_mapper(table = "posts")]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub images: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct Posts {
    pub posts: Vec<Post>,
}

#[derive(Deserialize, PostgresMapper, Serialize, Debug)]
#[pg_mapper(table = "posts")]
pub struct PostJsonResponse {
    pub id: i32,
    pub username: String,
    pub title: String,
    pub content: String,
    pub images: Option<Vec<String>>,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct PostsJsonResponse {
    pub posts: Vec<PostJsonResponse>,
}

impl From<Row> for Post {
    fn from(row: Row) -> Self {
        Post {
            id: row.get("id"),
            user_id: row.get("user_id"),
            title: row.get("title"),
            content: row.get("content"),
            images: row.get("images"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

impl From<Row> for PostJsonResponse {
    fn from(row: Row) -> Self {
        let created_at: DateTime<Utc> = row.get("created_at");
        let created_at_str = created_at.format("%Y-%m-%d").to_string();

        PostJsonResponse {
            id: row.get("id"),
            username: row.get("username"),
            title: row.get("title"),
            content: row.get("content"),
            images: row.get("images"),
            created_at: created_at_str,
        }
    }
}
