// use axum::http;
use axum::{http, extract::{Request, Path}};
// use uuid::Uuid;

// pub async fn echo(Path(user_id): Path<Uuid> ) -> Result<axum::Json<&'static str>, http::StatusCode> {
//     // println!("{:?}\n{:?}", r, user_id);
//     return Ok(axum::Json("Hi There!"));
// }

pub async fn main_page() -> Result<axum::response::Html<String>, http::StatusCode> {
    Err(http::StatusCode::INTERNAL_SERVER_ERROR)
} 

pub async fn tasks_page() -> Result<axum::response::Html<String>, http::StatusCode> {
    Err(http::StatusCode::INTERNAL_SERVER_ERROR)
}
