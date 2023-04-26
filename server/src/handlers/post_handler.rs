use crate::handlers::error_handler::AppError;
use actix_web::{get, web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};
use crate::entities::post::{PostJsonResponse, PostsJsonResponse};
use crate::entities::pagination::Pagination;

// The `total_pages` function returns the total number of pages:
#[get("/api/posts/total_pages/{page_size}")]
async fn total_pages(
    db_pool: web::Data<Pool>,
    page_size: web::Path<i64>,
) -> Result<HttpResponse, Error> {
    let page_size = page_size.into_inner();

    let client: Client = db_pool
        .get()
        .await
        .map_err(|e| AppError::PoolError(e))?;
    let total_posts_row = client
        .query_one("SELECT COUNT(*) FROM posts", &[])
        .await
        .map_err(|e| AppError::PGError(e))?;

    let total_posts: i64 = total_posts_row.get(0);
    let total_pages = (total_posts as f64 / page_size as f64).ceil() as i64;

    Ok(HttpResponse::Ok().json(total_pages))
}

// The `get_all` function supports pagination using `page` and `page_size` query params:
// - `page`: Specifies the page number (default: 1).
// - `page_size`: Specifies the number of posts per page (default: 10).
// The function calculates the `offset` as `page_size * (page - 1)`.
#[get("/api/posts/get_all")]
async fn get_all(
    db_pool: web::Data<Pool>,
    web::Query(pagination): web::Query<Pagination>,
) -> Result<HttpResponse, Error> {
    let page = pagination.page.unwrap_or(1);
    let page_size = pagination.page_size.unwrap_or(10);
    let offset = (page - 1) * page_size;

    let client: Client = db_pool
        .get()
        .await
        .map_err(|e| AppError::PoolError(e))?;

    let statement = client
        .prepare("SELECT p.id, u.name as username, p.title, p.content, p.images, p.created_at FROM posts p INNER JOIN users u ON p.user_id = u.id ORDER BY p.created_at DESC LIMIT $1 OFFSET $2")
        .await
        .map_err(|e| AppError::PGError(e))?;

    let rows = client
        .query(&statement, &[&page_size, &offset])
        .await
        .map_err(|e| AppError::PGError(e))?;

    let posts: Vec<PostJsonResponse> = rows.into_iter().map(PostJsonResponse::from).collect();

    Ok(HttpResponse::Ok().json(PostsJsonResponse { posts }))
}

// The `get_by_id` function fetches a post by its `id`:
#[get("/api/posts/{id}")]
async fn get_by_id(
    db_pool: web::Data<Pool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let id = id.into_inner();
    let client: Client = db_pool
        .get()
        .await
        .map_err(|e| AppError::PoolError(e))?;

    let statement = client
        .prepare("SELECT p.id, u.name as username, p.title, p.content, p.images, p.created_at FROM posts p INNER JOIN users u ON p.user_id = u.id WHERE p.id = $1")
        .await
        .map_err(|e| AppError::PGError(e))?;

    let row = client
        .query_one(&statement, &[&id])
        .await
        .map_err(|e| AppError::PGError(e))?;

    let post = PostJsonResponse::from(row);

    Ok(HttpResponse::Ok().json(post))
}

// #[post("/api/post/store")]
// async fn store(db_pool: &Pool, user_id: i32, title: &str, content: &str, images: &[&str]) -> Result<(), tokio_postgres::Error> {
//     let client: Client = db_pool
//         .get()
//         .await
//         .map_err(|e| AppError::PoolError(e))?;

//     let statement = client.prepare("INSERT INTO posts (user_id, title, content, images) VALUES ($1, $2, $3, $4)").await?;

//     let images_array: Vec<&str> = images.iter().map(|&image| image).collect();
//     client.execute(&statement, &[&user_id, &title, &content, &images_array]).await?;

//     Ok(())
// }


// #[cfg(test)]
// mod tests {
//     use actix_web::{http::StatusCode, test, web, App, HttpServer};
//     use deadpool_postgres::Pool;

//     use super::*;

//     // Helper function to create a test app
//     fn create_test_app() -> std::io::Result<()> {
//         // TODO
//     }

//     #[actix_web::test]
//     async fn test_get_all() {
//         let app = test::init_service(create_test_app()).await;
//         let req = test::TestRequest::get().uri("/api/posts/get_all").to_request();
//         let resp = test::call_service(&app, req).await;

//         assert_eq!(resp.status(), StatusCode::OK);
//     }

//     #[actix_web::test]
//     async fn test_get_by_id() {
//         let app = test::init_service(create_test_app()).await;
//         let req = test::TestRequest::get().uri("/api/posts/get_by_id/1").to_request();
//         let resp = test::call_service(&app, req).await;

//         assert_eq!(resp.status(), StatusCode::OK);
//     }
// }
