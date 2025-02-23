use axum::{ Router, routing::{get, post, delete, patch}};
// pub mod error;
// pub use error::AppError;
// mod handle;
// pub use handle::*;
// mod error;
// use error::AppError;
pub mod handle;
pub use handle::*;
pub mod error;
pub use error::AppError;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/files/{*path}", get(get_file))
        .route("/files/{*path}", post(create_file))
        .route("/files/{*path}", delete(delete_file))
        .route("/files/{*path}", patch(update_file));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    println!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;
    Ok(())
}


pub fn setup_router() -> Router {
    Router::new()
        .route("/files/{*path}", get(get_file))
        .route("/files/{*path}", post(create_file))
        .route("/files/{*path}", delete(delete_file))
        .route("/files/{*path}", patch(update_file))
}