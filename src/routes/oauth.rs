use crate::{auth, Request};
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

            let sid = auth::create_sid(entity.id)?;
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

            let sid = auth::parse_sid(sid.unwrap().value());
            if sid.is_err() {
                return Ok(oauth_redirect!("error=unauthorized_client"));
            }

            let sid = sid.unwrap();
            dbg!(&sid.sub);

            let code = auth::create_authorization_code(sid.sub)?;

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

            let user_id = auth::parse_authorization_code(entity.code);
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

            let access_token = auth::create_access_token(user_id, entity.client_id)?;

            let res = oauth_response!(StatusCode::Ok, access_token);

            Ok(res)
        }
        Err(_) => Ok(oauth_response!(
            StatusCode::BadRequest,
            json!({ "error": "invalid_request" }),
        )),
    }
}
