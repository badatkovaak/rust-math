pub mod handlers;


use tower_http::services::{ServeFile, ServeDir};
use axum::{
    self,
    routing::get,
    Router,
};


#[tokio::main]
async fn main() {
    let app = Router::new()
        // .nest_service("/", service)
        // .route("/", get(handlers::main_page))
        .nest_service("/", ServeFile::new("ui/main.html"))
        // .nest_service("", service)
        .nest_service("/tasks", ServeFile::new("ui/tasks.html"))
        .nest_service("/about", ServeFile::new("ui/about.html"))    
        .nest_service("/ui", ServeDir::new("ui"));
        // .route("/tasks/", get(handlers::tasks_page));
        // .route("/", get(|| async { "Hello, World!" }))

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
