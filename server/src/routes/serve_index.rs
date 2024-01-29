use axum::{
    response::Response,
    http::StatusCode, 
    body::Body, 
    extract::State
};
use tera::Context;
use tower_cookies::Cookies;

use crate::{data::TEMPLATES, state::AppState};

pub async fn serve_index(cookies: Cookies, State(state): State<AppState>) -> Response<Body> {
    let user = crate::auth::get_user_from_session_cookie(state.mongo.clone(), cookies.clone()).await;
    if user.is_none() {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header("Location", "/auth")
            .body(Body::from("<!DOCTYPE html><html><head><title>Redirecting to login...</title><script>setTimeout(()=>{{location.href=\"/auth\"}}, 3000);</script></head><body><h1>Redirecting to login...</h1><p><a href=\"/auth\">Not Working?</a></p></body></html>"))
            .unwrap()
    }
    
    let context = Context::new();
    let data = match TEMPLATES.render("index.html", &context) {
        Ok(s) => s,
        Err(e) => {
            tracing::warn!("Parsing error(s): {}", e);
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("Content-Type", "text/plain")
                .body(Body::from("Error parsing template"))
                .unwrap();
        }
    };
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Body::from(data))
        .unwrap()
}