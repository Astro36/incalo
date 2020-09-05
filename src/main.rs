use sqlx::mysql::MySqlPool;
use std::env;

pub mod auth;
pub mod model;
pub mod routes;

#[derive(Clone)]
pub struct Config {
    pub ID_TOKEN_ISSUER: String,
    pub ID_TOKEN_EXPIRES_IN: usize,
    pub ID_TOKEN_SECRET: String,
    pub MYSQL_HOST: String,
    pub MYSQL_PORT: u16,
    pub MYSQL_DATABASE: String,
    pub MYSQL_USER: String,
    pub MYSQL_PASSWORD: String,
    pub OAUTH_DEFAULT_REDIRECT_URI: String,
    pub SID_EXPIRES_IN: usize,
}

#[derive(Clone)]
pub struct State {
    config: Config,
    pool: MySqlPool,
}

pub type Request = tide::Request<State>;
pub type Server = tide::Server<State>;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let config = Config {
        ID_TOKEN_ISSUER: env::var("ID_TOKEN_ISSUER")?,
        ID_TOKEN_EXPIRES_IN: env::var("ID_TOKEN_EXPIRES_IN")?.parse()?,
        ID_TOKEN_SECRET: env::var("ID_TOKEN_SECRET")?,

        MYSQL_HOST: env::var("MYSQL_HOST")?,
        MYSQL_PORT: env::var("MYSQL_PORT")?.parse()?,
        MYSQL_DATABASE: env::var("MYSQL_DATABASE")?,
        MYSQL_USER: env::var("MYSQL_USER")?,
        MYSQL_PASSWORD: env::var("MYSQL_PASSWORD")?,

        OAUTH_DEFAULT_REDIRECT_URI: env::var("OAUTH_DEFAULT_REDIRECT_URI")?,

        SID_EXPIRES_IN: env::var("SID_EXPIRES_IN")?.parse()?,
    };

    let mysql_uri = format!(
        "mysql://{}:{}@{}:{}/{}",
        &config.MYSQL_USER,
        &config.MYSQL_PASSWORD,
        &config.MYSQL_HOST,
        &config.MYSQL_PORT,
        &config.MYSQL_DATABASE,
    );
    let pool = MySqlPool::connect(&mysql_uri).await?;

    let state = State { config, pool };

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
