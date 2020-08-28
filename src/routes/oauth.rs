use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use tide::http::{headers, mime, Cookie};
use tide::{Redirect, Request, Response, StatusCode};

macro_rules! oauth_redirect {
    ($redirect_uri:expr, $query:expr $(,)?) => {
        Redirect::new(format!("{}?{}", $redirect_uri, $query)).into()
    };
    ($query:expr) => {
        oauth_redirect!(env::var("OAUTH_DEFAULT_REDIRECT_URI").unwrap(), $query)
    };
}

macro_rules! oauth_response {
    ($status_code:expr, $body:expr $(,)?) => {
        Response::builder($status_code)
            .header(headers::CACHE_CONTROL, "no-store")
            .header(headers::PRAGMA, "no-cache")
            .content_type(mime::JSON)
            .body($body)
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
pub async fn login(mut req: Request<()>) -> tide::Result {
    match req.body_form::<LoginEntity>().await {
        Ok(entity) => {
            dbg!(&entity.id, &entity.password);

            let sid = "aaa.bbb.ccc";

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
pub async fn request_authorization_code(req: Request<()>) -> tide::Result {
    match req.query::<RequestCodeEntity>() {
        Ok(entity) => {
            if entity.response_type != "code" {
                return Ok(oauth_redirect!("error=unsupported_response_type"));
            }

            let code = "aaa.bbb.ccc";

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
pub async fn request_access_token(mut req: Request<()>) -> tide::Result {
    match req.body_form::<RequestTokenEntity>().await {
        Ok(entity) => {
            if entity.grant_type != "authorization_code" {
                return Ok(oauth_response!(
                    StatusCode::BadRequest,
                    json!({ "error": "unsupported_grant_type" }),
                ));
            }

            let access_token = "aaa.bbb.ccc";
            let expires_in = 3600;

            let res = oauth_response!(
                StatusCode::Ok,
                json!({
                    "access_token": access_token,
                    "token_type": "bearer",
                    "expires_in": expires_in,
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
