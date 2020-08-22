use async_std::io;
use tide::log;

mod routes;
mod services;

#[async_std::main]
async fn main() -> io::Result<()> {
    #[cfg(debug_assertions)]
    log::start();

    let mut app = tide::new();

    app.at("/api/oauth").post(routes::api::sign_in);
    app.at("/api/oauth/token")
        .post(routes::api::create_access_token);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
