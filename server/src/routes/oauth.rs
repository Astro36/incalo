use serde::{Deserialize, Serialize};
use serde_json::json;
use tide::http::{headers, mime, Cookie};
use tide::{Redirect, Request, Response, StatusCode};

macro_rules! oauth_error {
    ($error:literal) => {
        Response::builder(StatusCode::BadRequest)
            .header(headers::CACHE_CONTROL, "no-store")
            .header(headers::PRAGMA, "no-cache")
            .content_type(mime::FORM)
            .body(format!("error={}", $error))
            .build()
    };
}

#[derive(Debug, Deserialize, Serialize)]
struct SignInEntity {
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
pub async fn sign_in(mut req: Request<()>) -> tide::Result {
    match req.body_form::<SignInEntity>().await {
        Ok(entity) => {
            dbg!(&entity.id, &entity.password);

            let sid = "aaa.bbb.ccc";

            let cookie = Cookie::build("incalo_sid", sid)
                .secure(!cfg!(debug_assertions))
                .finish();

            let mut res = Response::builder(StatusCode::Ok)
                .header(headers::CACHE_CONTROL, "no-store")
                .header(headers::PRAGMA, "no-cache")
                .content_type(mime::JSON)
                .body(json!({"consent": true }))
                .build();
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
                return Ok(oauth_error!("unsupported_response_type"));
            }

            let code = "aaa.bbb.ccc";

            let location = format!(
                "{}?code={}&state={}",
                entity.redirect_uri, code, entity.state
            );

            Ok(Redirect::new(location).into())
        }
        Err(_) => Ok(oauth_error!("invalid_request")),
    }
}

// POST /api/oauth/token
pub async fn request_access_token(mut req: Request<()>) -> tide::Result {
    match req.body_form::<RequestTokenEntity>().await {
        Ok(entity) => {
            if entity.grant_type != "authorization_code" {
                return Ok(oauth_error!("unsupported_grant_type"));
            }

            let access_token = "aaa.bbb.ccc";
            let expires_in = 3600;

            let body = json!({
                "access_token": access_token,
                "token_type": "bearer",
                "expires_in": expires_in,
            });

            let res = Response::builder(StatusCode::Ok)
                .header(headers::CACHE_CONTROL, "no-store")
                .header(headers::PRAGMA, "no-cache")
                .content_type(mime::JSON)
                .body(body)
                .build();

            Ok(res)
        }
        Err(_) => Ok(oauth_error!("invalid_request")),
    }
}
