use std::env;

#[derive(Clone)]
pub struct Config {
    pub id_token_issuer: String,
    pub id_token_expires_in: usize,
    pub id_token_secret: String,

    pub mysql_host: String,
    pub mysql_port: u16,
    pub mysql_database: String,
    pub mysql_user: String,
    pub mysql_password: String,

    pub oauth_default_redirect_uri: String,

    pub sid_expires_in: usize,
}

impl Config {
    pub fn load() -> tide::Result<Self> {
        Ok(Self {
            id_token_issuer: env::var("ID_TOKEN_ISSUER")?,
            id_token_expires_in: env::var("ID_TOKEN_EXPIRES_IN")?.parse()?,
            id_token_secret: env::var("ID_TOKEN_SECRET")?,

            mysql_host: env::var("MYSQL_HOST")?,
            mysql_port: env::var("MYSQL_PORT")?.parse()?,
            mysql_database: env::var("MYSQL_DATABASE")?,
            mysql_user: env::var("MYSQL_USER")?,
            mysql_password: env::var("MYSQL_PASSWORD")?,

            oauth_default_redirect_uri: env::var("OAUTH_DEFAULT_REDIRECT_URI")?,

            sid_expires_in: env::var("SID_EXPIRES_IN")?.parse()?,
        })
    }
}
