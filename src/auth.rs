use chrono::prelude::*;
use chrono::Duration;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTokenPayload {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionIDPayload {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: i64,
    pub iat: i64,
}

pub fn create_access_token(user_id: String, client_id: String) -> tide::Result<String> {
    let issuer = env::var("JWT_ACCESS_TOKEN_ISSUER")?;
    let expired = env::var("JWT_ACCESS_TOKEN_EXPIRED")?.parse()?;
    let secret = env::var("JWT_ACCESS_TOKEN_SECRET")?;

    let issued_at = Utc::now();
    let expiration_time = issued_at + Duration::seconds(expired);

    let header = Header::default();
    let payload = AccessTokenPayload {
        iss: issuer,
        sub: user_id,
        aud: client_id,
        exp: expiration_time.timestamp(),
        iat: issued_at.timestamp(),
    };
    let key = EncodingKey::from_secret(secret.as_bytes());

    Ok(jsonwebtoken::encode(&header, &payload, &key)?)
}

pub fn create_authorization_code(_user_id: String) -> tide::Result<String> {
    Ok("qwertyuiop1234567890".to_string())
}

pub fn create_sid(user_id: String) -> tide::Result<String> {
    let issuer = env::var("JWT_SID_ISSUER")?;
    let audience = env::var("JWT_SID_AUDIENCE")?;
    let expired = env::var("JWT_SID_EXPIRED")?.parse()?;
    let secret = env::var("JWT_SID_SECRET")?;

    let issued_at = Utc::now();
    let expiration_time = issued_at + Duration::seconds(expired);

    let header = Header::default();
    let payload = SessionIDPayload {
        iss: issuer,
        sub: user_id,
        aud: audience,
        exp: expiration_time.timestamp(),
        iat: issued_at.timestamp(),
    };
    let key = EncodingKey::from_secret(secret.as_bytes());

    Ok(jsonwebtoken::encode(&header, &payload, &key)?)
}

pub fn parse_access_token(access_token: &str) -> tide::Result<AccessTokenPayload> {
    let secret = env::var("JWT_ACCESS_TOKEN_SECRET").unwrap();
    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::default();

    Ok(jsonwebtoken::decode::<AccessTokenPayload>(access_token, &key, &validation)?.claims)
}

pub fn parse_authorization_code(_code: String) -> tide::Result<String> {
    Ok("hello_incalo".to_string())
}

pub fn parse_sid(sid: &str) -> tide::Result<SessionIDPayload> {
    let secret = env::var("JWT_SID_SECRET").unwrap();
    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::default();

    Ok(jsonwebtoken::decode::<SessionIDPayload>(sid, &key, &validation)?.claims)
}
