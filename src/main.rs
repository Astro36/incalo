use axum::routing::get;
use axum::{AddExtensionLayer, Router, Server};
use qp_postgres::tokio_postgres::NoTls;
use qp_postgres::PgPool;

const DB_URI: &str = "postgresql://postgres:postgres@localhost/incalo";
const SERVER_ADDRESS: &str = "0.0.0.0:3000";

type DbPool = PgPool<NoTls>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = DB_URI.parse().unwrap();
    let pool = qp_postgres::connect(config, NoTls, 8);
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(AddExtensionLayer::new(pool));
    let addr = SERVER_ADDRESS.parse().unwrap();
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
