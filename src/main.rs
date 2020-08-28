use async_std::io;
use tide::log;

mod routes;

#[async_std::main]
async fn main() -> io::Result<()> {
    #[cfg(debug_assertions)]
    log::start();

    let mut app = tide::new();

    app.at("/api/oauth").post(routes::oauth::login);
    app.at("/api/oauth/authorize")
        .get(routes::oauth::request_authorization_code);
    app.at("/api/oauth/token")
        .post(routes::oauth::request_access_token);

    app.at("/api/users").post(routes::users::create_account);
    app.at("/api/users").delete(routes::users::delete_account);
    app.at("/api/users/logout").post(routes::users::logout);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
