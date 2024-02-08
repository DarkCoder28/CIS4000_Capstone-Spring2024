use std::env;

use axum::{
    routing::{get, post}, 
    Router, 
};

use gwynedd_valley_server::{auth::{self, finish_authentication, finish_register, logout, start_authentication, start_register}, mongo::connect_mongo, routes::{handle_session_ws::ws_session_handler, serve_auth::serve_auth, serve_index::serve_index, static_files::{serve_robots, serve_static}}, state::AppState};
use tower_cookies::CookieManagerLayer;
use tower_sessions::{
    cookie::{time::Duration,SameSite},
    Expiry, SessionManagerLayer,
};
use tower_sessions_moka_store::MokaStore;

#[tokio::main]
async fn main() {
    let tracing_sub = tracing_subscriber::FmtSubscriber::new();
    let _ = tracing::subscriber::set_global_default(tracing_sub);

    // Setup Session Store
    let session_store = MokaStore::new(None);
    // let session_service = ServiceBuilder::new()
    //     .layer(HandleErrorLayer::new(|_: BoxError| async {
    //         StatusCode::BAD_REQUEST
    //     }))
    //     .layer(
    //         SessionManagerLayer::new(session_store)
    //             .with_name("webauthn")
    //             .with_same_site(SameSite::Strict)
    //             // If INSECURE is not defined, this will error, causing it to be secure
    //             .with_secure(env::var("INSECURE").is_err())
    //             .with_expiry(Expiry::OnInactivity(Duration::seconds(360))),
    //     );
    let session_layer = SessionManagerLayer::new(session_store)
        .with_name("webauthn")
        .with_same_site(SameSite::Strict)
        // If INSECURE is not defined, this will error, causing it to be secure
        .with_secure(env::var("INSECURE").is_err())
        .with_expiry(Expiry::OnInactivity(Duration::seconds(360)));

    // Setup MongoDB Connection
    let mongodb_connection = connect_mongo().await;
    if mongodb_connection.is_err() {
        panic!("Error connecting to MongoDB\nError:\n{}", mongodb_connection.unwrap_err().to_string());
    }

    // Create App State
    let app_state = AppState {
        mongo: mongodb_connection.unwrap(),
        webauthn: auth::new()
    };
    // let cleanup_mongo = app_state.mongo.clone();

    let app = Router::new()
        .route("/", get(serve_index))
        .route("/robots.txt", get(serve_robots))
        .route("/static/*path", get(serve_static))
        // Authorization
        .route("/auth", get(serve_auth))
        .route("/api/auth/register_start/:username", post(start_register))
        .route("/api/auth/register_finish", post(finish_register))
        .route("/api/auth/login_start/:username", post(start_authentication))
        .route("/api/auth/login_finish", post(finish_authentication))
        .route("/api/auth/logout", post(logout))
        // API
        .route("/api/connect_session", get(ws_session_handler))
        // Required Service Workers
        .layer(session_layer)
        .layer(CookieManagerLayer::new())
        .with_state(app_state);

    // Start Listening
    tracing::info!("Listening on 0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}