use serde::Deserialize;
use tide::{Request, Response, StatusCode};

#[derive(Deserialize)]
struct SignIn {
    id: String,
    password: String,
}

// POST /api/oauth
pub async fn sign_in(mut req: Request<()>) -> tide::Result {
    match req.body_form::<SignIn>().await {
        Ok(auth) => {
            dbg!(&auth.id, &auth.password);
            let mut res = Response::new(StatusCode::Ok);
            res.append_header("Content-Type", "text/plain");
            Ok(res)
        }
        Err(_) => Ok(Response::new(StatusCode::BadRequest)),
    }
}

// POST /api/oauth/token
pub async fn create_access_token(req: Request<()>) -> tide::Result {
    let mut res = Response::new(StatusCode::Ok);
    res.append_header("Content-Type", "text/plain");
    Ok(res)
}
