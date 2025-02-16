use axum::{
    routing::get, 
    Router,
};

mod handle;
mod command;
use handle::handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await?;
        

    println!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;
    Ok(())
}