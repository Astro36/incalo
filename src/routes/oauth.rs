use crate::model::{Department, Gender, IdTokenPayload, User};
use crate::{auth, Request, State};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use tide::http::{headers, mime, Cookie};
use tide::{Redirect, Response, StatusCode};

macro_rules! oauth_redirect {
    ($redirect_uri:expr, $query:expr $(,)?) => {
        Redirect::new(format!("{}?{}", $redirect_uri, $query)).into()
    };
    ($query:expr) => {
        oauth_redirect!(env::var("OAUTH_DEFAULT_REDIRECT_URI")?, $query)
    };
}

macro_rules! oauth_response {
    ($status_code:expr, $json:expr $(,)?) => {
        Response::builder($status_code)
            .header(headers::CACHE_CONTROL, "no-store")
            .header(headers::PRAGMA, "no-cache")
            .content_type(mime::JSON)
            .body($json)
            .build()
    };
}

#[derive(Debug, Deserialize, Serialize)]
struct LoginEntity {
    id: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct RequestCodeEntity {
    response_type: String,
    client_id: String,
    redirect_uri: String,
    state: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct RequestTokenEntity {
    grant_type: String,
    code: String,
    redirect_uri: String,
    client_id: String,
}

// POST /api/oauth
pub async fn login(mut req: Request) -> tide::Result {
    match req.body_form::<LoginEntity>().await {
        Ok(entity) => {
            dbg!(&entity.id, &entity.password);

            let sid = "qwertyuiop1234567890".to_string();
            dbg!(&sid);

            let cookie = Cookie::build("incalo_sid", sid)
                .secure(!cfg!(debug_assertions))
                .finish();

            let mut res = oauth_response!(StatusCode::Ok, json!({ "consent": true }));
            res.insert_cookie(cookie);

            Ok(res)
        }
        Err(_) => Ok(Response::new(StatusCode::BadRequest)),
    }
}

// GET /api/oauth/authorize
pub async fn request_authorization_code(req: Request) -> tide::Result {
    match req.query::<RequestCodeEntity>() {
        Ok(entity) => {
            if entity.response_type != "code" {
                return Ok(oauth_redirect!("error=unsupported_response_type"));
            }

            let sid = req.cookie("incalo_sid");
            if sid.is_none() {
                return Ok(oauth_redirect!("error=unauthorized_client"));
            }

            let user_id: Result<String, async_std::io::Error> = Ok("".to_string());
            if user_id.is_err() {
                return Ok(oauth_redirect!("error=unauthorized_client"));
            }

            let user_id = user_id.unwrap();
            dbg!(&user_id);

            let code = "qwertyuiop1234567890".to_string();

            let res = oauth_redirect!(
                entity.redirect_uri,
                format!("code={}&state={}", code, entity.state),
            );

            Ok(res)
        }
        Err(_) => Ok(oauth_redirect!("error=invalid_request")),
    }
}

// POST /api/oauth/token
pub async fn request_access_token(mut req: Request) -> tide::Result {
    match req.body_form::<RequestTokenEntity>().await {
        Ok(entity) => {
            if entity.grant_type != "authorization_code" {
                return Ok(oauth_response!(
                    StatusCode::BadRequest,
                    json!({ "error": "unsupported_grant_type" }),
                ));
            }

            let user_id: Result<String, async_std::io::Error> = Ok("hello".to_string());
            if user_id.is_err() {
                return Ok(oauth_response!(
                    StatusCode::BadRequest,
                    json!({ "error": "invalid_grant" }),
                ));
            }

            let user_id = user_id.unwrap();
            if false {
                return Ok(oauth_response!(
                    StatusCode::Unauthorized,
                    json!({ "error": "invalid_client" }),
                ));
            }

            let State { config, pool } = req.state();

            let user = User {
                id: user_id,
                email: "hello@example.com".to_string(),
                name: "Hello".to_string(),
                gender: Gender::HIDDEN,
                department: Department::UNK,
            };

            let payload = IdTokenPayload::new(
                config.ID_TOKEN_ISSUER.clone(),
                "client_id".to_string(),
                config.ID_TOKEN_EXPIRES_IN,
                user,
            );
            let id_token = auth::encode_id_token(&payload, &config.ID_TOKEN_SECRET)?;

            let access_token = "qwertyuiop1234567890".to_string();
            let refresh_token = "qwertyuiop1234567890".to_string();

            let res = oauth_response!(
                StatusCode::Ok,
                json!({
                    "access_token": access_token,
                    "token_type": "Bearer",
                    "refresh_token": refresh_token,
                    "expires_in": config.ID_TOKEN_EXPIRES_IN,
                    "id_token": id_token,
                }),
            );

            Ok(res)
        }
        Err(_) => Ok(oauth_response!(
            StatusCode::BadRequest,
            json!({ "error": "invalid_request" }),
        )),
    }
}
