use axum::{
    response::Response,
    http::StatusCode, 
    body::Body, extract::State
};
use tera::Context;
use tower_cookies::Cookies;

use crate::{data::TEMPLATES, state::AppState};

pub async fn serve_auth(cookies: Cookies, State(state): State<AppState>) -> Response<Body> {
    let uploader = crate::auth::get_user_from_session_cookie(state.mongo.clone(), cookies.clone()).await;
    //
    let mut context = Context::new();
    context.insert("user", &uploader);
    let data = match TEMPLATES.render("auth.html", &context) {
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