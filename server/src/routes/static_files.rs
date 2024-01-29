use axum::{
    response::Response,
    http::StatusCode, 
    body::Body, 
    extract::Path
};

use crate::data::STATIC_DIR;

pub async fn serve_static(Path(path): Path<String>) -> Response<Body> {
    let mime = mime_guess::from_path(&path).first();
    if mime.is_none() {
        return Response::builder()
            .status(StatusCode::IM_A_TEAPOT)
            .header("Content-Type", "text/plain")
            .body(Body::from("Error parsing MIME type from request URL"))
            .unwrap();
    }
    let mime = mime.unwrap().essence_str().to_string();
    let file = STATIC_DIR.get_file(path);
    if file.is_none() {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("Content-Type", "text/plain")
            .body(Body::from("Requested file does not exist."))
            .unwrap();
    }
    let file = file.unwrap().contents();
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", mime)
        .body(Body::from(file))
        .unwrap()
}

pub async fn serve_robots() -> Response<Body> {
    serve_static(Path("robots.txt".to_string())).await
}