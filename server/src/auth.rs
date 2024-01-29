use std::{sync::Arc, str::FromStr};
use axum::{extract::{State, Path}, response::{IntoResponse, Response}, Json, http::StatusCode, body::Body};
use bson::doc;
use chrono::Utc;
use mongodb::{Client, results::InsertOneResult};
use serde::{Serialize, Deserialize};
use tower_cookies::{Cookies, Cookie, cookie::SameSite};
use tower_sessions::Session;
use tracing::error;
use uuid::Uuid;
use webauthn_rs::{Webauthn, prelude::{Url, Passkey, Base64UrlSafeData, RegisterPublicKeyCredential, PasskeyRegistration, PublicKeyCredential, PasskeyAuthentication}, WebauthnBuilder};
use std::env;

use crate::auth_error::WebauthnError;
use crate::state::AppState;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub username: String,
    pub user_id: String,
    pub keys: Vec<String>,
}
impl User {
    #[allow(dead_code)]
    pub fn get_user_id(&self) -> Uuid {
        Uuid::from_str(&self.user_id).expect("Error parsing user ID")
    }
    #[allow(dead_code)]
    pub fn set_user_id(&mut self, id: Uuid) {
        self.user_id = id.to_string();
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct UserSession {
    pub user_id: String,
    pub session_id: String,
    pub creation_time: i64,
}
impl UserSession {
    #[allow(dead_code)]
    pub fn new(user_id: String) -> UserSession {
        UserSession {
            user_id,
            session_id: Uuid::new_v4().to_string(),
            creation_time: Utc::now().timestamp(),
        }
    }
    #[allow(dead_code)]
    pub fn get_user_id(&self) -> Uuid {
        Uuid::from_str(&self.user_id).expect("Error parsing user ID")
    }
}
const SESSION_COOKIE: &str = "cis4000-project-session";

pub fn new() -> Arc<Webauthn> {
    let rp_id = env::var("PK_ID").expect("You must set the RP_ID environment var!");
    let rp_origin = Url::parse(&env::var("PK_ORIGIN").expect("You must set the PK_ORIGIN environment var!")).expect("Invalid URL");
    let builder = WebauthnBuilder::new(&rp_id, &rp_origin).expect("Invalid configuration");
    let builder = builder.rp_name("CIS4000-Project");
    Arc::new(builder.build().expect("Invalid configuration"))
}





pub async fn start_register(session: Session, State(state): State<AppState>, Path(username): Path<String>) -> Response<Body> {
    let (mongo, webauthn) = (state.mongo.clone(), state.webauthn.clone());
    // Get user based on username (if they exist)
    let user = get_user_by_username(mongo, &username).await;
    // Generate new user id if they don't exist
    let user_id = match user.clone() { Some(x) => x.get_user_id(), None => Uuid::new_v4()};
    // Cleanup session
    let _ = session.remove_value("reg_state").await;
    // No idea how this works, but it was adapted from the example code
    let exclude_credentials: Option<Vec<Base64UrlSafeData>> = user.map(|user| user.keys.iter().map(|sk| (serde_json::from_str::<Passkey>(&sk)).expect("Error deserializing passkey").cred_id().clone()).collect());
    // Call the start registration function
    let res = match webauthn.start_passkey_registration(
        user_id, 
        &username, 
        &username, 
        exclude_credentials
    ) {
        Ok((ccr, reg_state)) => {
            session.insert("reg_state", (username, user_id, reg_state)).await.expect("Failed to insert reg state");
            serde_json::to_string(&ccr)
        },
        Err(_) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("Content-Type", "text/plain")
                .body(Body::from(""))
                .unwrap();
        }
    };
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Body::from(res.expect("Serialization Error")))
        .unwrap()
}

pub async fn finish_register(session: Session, State(state): State<AppState>, Json(reg): Json<RegisterPublicKeyCredential>) -> Result<impl IntoResponse, WebauthnError> {
    let (mongo, webauthn) = (state.mongo.clone(), state.webauthn.clone());
    // Get the registration state info from the server's session store
    let (username, user_id, reg_state): (String, Uuid, PasskeyRegistration) = match session.get("reg_state").await? {
        Some(x) => x,
        None => {
            return Err(WebauthnError::CorruptSession);
        }
    };
    // Cleanup the session
    let _ = session.remove_value("reg_state").await;
    let res = match webauthn.finish_passkey_registration(&reg, &reg_state) {
        Ok(sk) => {
            match get_user_by_username(mongo.clone(), &username).await {
                Some(_user) => {
                    // If the user already exists, fail the registration as the user is already there
                    return Err(WebauthnError::UserExists);
                },
                None => {
                    // If the user does not exist, register them
                    let key = serde_json::to_string(&sk).expect("Error serializing key");
                    let user = User { username: username, user_id: user_id.to_string(), keys: vec![key] };
                    match save_user_registration(mongo.clone(), user).await {
                        Ok(_) => (),
                        Err(_) => {
                            return Err(WebauthnError::UserExists);
                        }
                    }
                }
            }
            StatusCode::OK
        },
        Err(e) => {
            error!("{}", e);
            StatusCode::BAD_REQUEST
        }
    };
    Ok(res)
}



pub async fn start_authentication(session: Session, State(state): State<AppState>, Path(username): Path<String>) -> Result<impl IntoResponse, WebauthnError> {
    let _ = session.remove_value("auth_state").await;
    let user = get_user_by_username(state.mongo.clone(), &username).await.ok_or(WebauthnError::UserNotFound)?;
    let allow_creds = Vec::from_iter((&user.keys).into_iter().map(|key| serde_json::from_str::<Passkey>(&key).expect("Error deserializing passkey")));
    let res = match state.webauthn.start_passkey_authentication(&allow_creds) {
        Ok((rcr, auth_state)) => {
            session.insert("auth_state", (user.user_id, auth_state)).await.expect("Failed to insert authentication state to session");
            Json(rcr)
        },
        Err(_) => return Err(WebauthnError::Unknown)
    };
    Ok(res)
}

pub async fn finish_authentication(session: Session, cookies: Cookies, State(state): State<AppState>, Json(auth): Json<PublicKeyCredential>) -> Result<impl IntoResponse, WebauthnError> {
    let (user_id, auth_state): (Uuid, PasskeyAuthentication) = session.get("auth_state").await?.ok_or(WebauthnError::CorruptSession)?;
    let user = get_user_by_id(state.mongo.clone(), user_id.clone()).await.ok_or(WebauthnError::UserNotFound)?;
    let _ = session.remove_value("auth_state").await;
    match get_session_id_from_session_cookie(cookies.clone()) {
        Some(x) => {
            clear_user_session(state.mongo.clone(), x).await;
        },
        None => ()
    }
    cookies.remove(Cookie::new(SESSION_COOKIE, ""));
    let res = match state.webauthn.finish_passkey_authentication(&auth, &auth_state) {
        Ok(auth_result) => {
            if user.keys.len() <= 0 {
                return Err(WebauthnError::UserHasNoCredentials);
            }
            let mut keys = Vec::from_iter((&user.keys).into_iter().map(|key| serde_json::from_str::<Passkey>(&key).expect("Error deserializing passkey")));
            keys.iter_mut().for_each(|sk| {sk.update_credential(&auth_result);});
            let updated_keys = keys.iter().map(|key| serde_json::to_string(&key).expect("Error serializing key")).collect();
            update_user_cred_counter(state.mongo.clone(), user_id, updated_keys).await;
            let session_cookie = Cookie::build((SESSION_COOKIE, create_user_session(state.mongo.clone(), &user).await))
                .path("/")
                .secure(env::var("INSECURE").is_err())
                .http_only(true)
                .max_age(tower_cookies::cookie::time::Duration::hours(12))
                .same_site(SameSite::Strict)
                .build();
            cookies.add(session_cookie);
            StatusCode::OK
        },
        Err(_) => StatusCode::BAD_REQUEST
    };
    Ok(res)
}

pub async fn logout(cookies: Cookies, State(state): State<AppState>) -> Result<impl IntoResponse, WebauthnError> {
    let session_id = match get_session_id_from_session_cookie(cookies.clone()) {
        Some(sid) => sid,
        None => {
            return Ok(StatusCode::OK);
        }
    };
    clear_user_session(state.mongo, session_id).await;
    cookies.remove(Cookie::new(SESSION_COOKIE, ""));
    Ok(StatusCode::OK)
}





async fn get_user_by_username(mongo: Client, username: &str) -> Option<User> {
    let user_collection = mongo
        .database("recipe-book")
        .collection::<User>("users");

    let user_res = user_collection.find_one(doc! {"username": username}, None).await;
    if user_res.is_err() {
        error!("Error getting user");
        return None;
    }
    user_res.expect("Pre-filtered exception")
}

pub async fn get_user_by_id(mongo: Client, user_id: Uuid) -> Option<User> {
    let user_collection = mongo
        .database("recipe-book")
        .collection::<User>("users");

    let user_res = user_collection.find_one(doc!{"user_id": user_id.to_string()}, None).await;
    if user_res.is_err() {
        let err = user_res.expect_err("").to_string();
        error!("Error getting user: {}", err);
        return None;
    }
    user_res.expect("Pre-filtered exception")
}

async fn save_user_registration(mongo: Client, user: User) -> Result<InsertOneResult, mongodb::error::Error> {
    let collection = mongo.database("recipe-book").collection::<User>("users");
    collection.insert_one(user, None).await
}

async fn update_user_cred_counter(mongo: Client, user_id: Uuid, updated_keys: Vec<String>) {
    let user_collection = mongo.database("recipe-book").collection::<User>("users");
    let update = doc!{
        "$set": {
            "keys": updated_keys
        }
    };
    let _ = user_collection.update_one(doc!{"user_id": user_id.to_string()}, update, None).await;
}

async fn create_user_session(mongo: Client, user: &User) -> String {
    let new_session = UserSession::new(user.user_id.clone());
    let collection = mongo.database("recipe-book").collection::<UserSession>("sessions");
    collection.insert_one(&new_session, None).await.expect("Session failed to create");
    new_session.session_id.to_string()
}


fn get_session_id_from_session_cookie(cookies: Cookies) -> Option<Uuid> {
    cookies.get(SESSION_COOKIE).and_then(|c|Some(Uuid::from_str(c.value()).expect("Error parsing session id")))
}
async fn get_user_id_from_session_id(mongo: Client, session_id: Uuid) -> Option<Uuid> {
    let collection = mongo.database("recipe-book").collection::<UserSession>("sessions");
    let user_session = collection.find_one(doc!{"session_id": session_id.to_string()}, None).await.expect("Database connection error");
    user_session.and_then(|session|Some(session.get_user_id()))
}

pub async fn get_user_id_from_session_cookie(mongo:Client, cookies: Cookies) -> Option<Uuid> {
    match get_session_id_from_session_cookie(cookies.clone()) {
        Some(sid) => {
            get_user_id_from_session_id(mongo, sid).await
        },
        None => None
    }
}
pub async fn get_user_from_session_cookie(mongo:Client, cookies: Cookies) -> Option<User> {
    match get_session_id_from_session_cookie(cookies.clone()) {
        Some(sid) => {
            match get_user_id_from_session_id(mongo.clone(), sid).await {
                Some(uid) => {
                    get_user_by_id(mongo, uid).await
                },
                None => None
            }
        },
        None => None
    }
}


async fn clear_user_session(mongo: Client, session_id: Uuid) {
    let collection = mongo.database("recipe-book").collection::<UserSession>("sessions");
    collection.delete_one(doc!{"session_id": session_id.to_string()}, None).await.expect("Failed to delete user session");
}