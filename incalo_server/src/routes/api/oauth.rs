use serde::{Deserialize, Serialize};
use serde_json::json;
use tide::http::{mime, Cookie};
use tide::{Request, Response, StatusCode};

#[derive(Debug, Deserialize, Serialize)]
struct SignIn {
    id: String,
    password: String,
}

// POST /api/oauth
pub async fn sign_in(mut req: Request<()>) -> tide::Result {
    match req.body_form::<SignIn>().await {
        Ok(auth) => {
            dbg!(&auth.id, &auth.password);

            let sid = "aaa.bbb.ccc";

            let cookie = Cookie::build("incalo_sid", sid)
                .secure(!cfg!(debug_assertions))
                .finish();

            let mut res = Response::new(StatusCode::Ok);
            res.insert_cookie(cookie);

            Ok(res)
        }
        Err(_) => Ok(Response::new(StatusCode::BadRequest)),
    }
}

// POST /api/oauth/token
pub async fn create_access_token(mut _req: Request<()>) -> tide::Result {
    let access_token = "aaa.bbb.ccc";
    let expires_in = 3600;

    let body = json!({
        "access_token": access_token,
        "token_type": "bearer",
        "expires_in": expires_in,
    });

    let res = Response::builder(StatusCode::Ok)
        .content_type(mime::JSON)
        .body(body)
        .build();

    Ok(res)
}
