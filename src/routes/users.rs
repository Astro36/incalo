use tide::http::Cookie;
use tide::{Request, Response, StatusCode};
use time::Duration;

// POST /api/users/logout
pub async fn logout(mut _req: Request<()>) -> tide::Result {
    let mut cookie = Cookie::named("incalo_sid");
    cookie.set_max_age(Duration::zero());

    let mut res = Response::new(StatusCode::Ok);
    res.insert_cookie(cookie);

    Ok(res)
}
