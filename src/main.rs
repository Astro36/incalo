use sqlx::mysql::MySqlPool;
use std::env;

pub mod auth;
pub mod routes;

#[derive(Clone)]
pub struct State {
    pool: MySqlPool,
}

pub type Request = tide::Request<State>;
pub type Server = tide::Server<State>;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mysql_uri = format!(
        "mysql://{}:{}@{}:{}/{}",
        env::var("MYSQL_USER")?,
        env::var("MYSQL_PASSWORD")?,
        env::var("MYSQL_HOST")?,
        env::var("MYSQL_PORT")?,
        env::var("MYSQL_DATABASE")?,
    );

    let pool = MySqlPool::connect(&mysql_uri).await?;
    let state = State { pool };

    let mut app = tide::with_state(state);

    #[cfg(debug_assertions)]
    tide::log::start();

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
