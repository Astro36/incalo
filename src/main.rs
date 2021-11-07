use axum::routing::get;
use axum::{AddExtensionLayer, Router, Server};
use deadpool_postgres::{Config, Runtime};
use std::net::SocketAddr;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();
    config.dbname = Some("incalo".to_string());
    let pool = config.create_pool(Some(Runtime::Tokio1), NoTls)?;
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(AddExtensionLayer::new(pool));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
