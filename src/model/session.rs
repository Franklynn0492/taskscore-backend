use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use okapi::openapi3::{Object, SecurityScheme, SecuritySchemeData, SecurityRequirement};
use rocket::{Request, http::Status, request::FromRequest, request::Outcome};
use rocket_okapi::{request::{OpenApiFromRequest, RequestHeaderInput}, gen::OpenApiGenerator};
use schemars::{JsonSchema};
use base64;

use crate::repository::neo4j_repsitory::Neo4JRepository;
use crate::repository::repository::Repository;

use super::{User, model::Entity};
use rand::Rng;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const PASSWORD_LEN: usize = 30;

#[derive(serde::Serialize, Clone, OpenApiFromRequest, JsonSchema)]
pub struct Session {
    pub id: String,
    pub user: Arc<Mutex<User>>,
    pub started: DateTime::<Utc>,
    pub refreshed: DateTime::<Utc>,
}

impl Session {
    pub fn new(user: Arc<Mutex<User>>) -> Session {
        let now = Utc::now();
        Session {
            id: Session::generate_session_id(),
            user,
            started: now.clone(),
            refreshed: now,
        }
    }

    fn generate_session_id() -> String {
        let mut rng = rand::thread_rng();

        let session_id: String = (0..PASSWORD_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        session_id
    }

    fn refresh(&mut self) {
        self.refreshed = Utc::now();
    }
}

impl Entity<String> for Session {
    fn get_id(&self) -> &String {
        &self.id
    }
}

#[async_trait]
impl <'a> FromRequest<'a> for Session {
    type Error = String;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let repository = request.rocket().state::<Neo4JRepository>();
        if repository.is_none() {
            return Outcome::Failure((Status::InternalServerError, "Missing status".to_owned()))
        }
        let repository = repository.unwrap();
        
        let cookie = request.cookies()
            .get("sid");
        if cookie.is_none() {
            return Outcome::Failure((Status::BadRequest, "No session provided".to_owned()))
        }
        let cookie = cookie.unwrap();

        let sid = cookie.value().to_owned();
        let session = repository.get_session(&sid).await;
        if session.is_none() {
            return Outcome::Failure((Status::Unauthorized, "Session not available".to_owned()))
        }

        let mut session = session.unwrap();
        session.refresh();
        Outcome::Success(session)
    }
}

#[derive(serde::Serialize, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: Option<String>,
}

#[async_trait]
impl<'a> FromRequest<'a> for LoginRequest {
    type Error = String;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let authorization_header_opt = request.headers().get_one("Authorization");
        match authorization_header_opt {
            Some(authorization_header) => {
                if !authorization_header.starts_with("Basic ") {
                    return Outcome::Failure((Status::BadRequest, "Authentcation header does not indicate Basic Authentication".to_owned()));
                }
                let login_data_encoded = authorization_header.split_at(5).1.trim();
                if login_data_encoded.len() == 0 {
                    return Outcome::Failure((Status::BadRequest, "Empty basic authentication header provided".to_owned()));
                }

                let decoded_res = base64::decode_config(login_data_encoded, base64::URL_SAFE);
                if decoded_res.is_err() {
                    return Outcome::Failure((Status::BadRequest, "Unable to decode basic authentication header".to_owned()));
                }

                let decoded_str_res = String::from_utf8(decoded_res.unwrap());
                if decoded_str_res.is_err() {
                    return Outcome::Failure((Status::BadRequest, "Unable to decode basic authentication header to utf8".to_owned()));
                }
                let decoded_str = decoded_str_res.unwrap();
                let split = decoded_str.split_once(":");
                let (username, password) = match split {
                    Some((u, p)) =>  (u.to_owned(), Some(p.to_owned())),
                    None => (decoded_str, None)
                };

                let login_request = LoginRequest {username, password};

                Outcome::Success(login_request)
            },
            None => Outcome::Failure((Status::Unauthorized, "No basic authentication header provided".to_owned()))
        }
    }
}
 

impl<'a> OpenApiFromRequest<'a> for LoginRequest {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        // Setup global requirement for Security scheme
        let security_scheme = SecurityScheme {
            description: Some(
                "Requires an HTTP Basic auth header. Try User: 'roterkohl'; Pwd: 'Flori1234'".to_owned(),
            ),
            // Setup data requirements.
            // In this case the header `Authorization: mytoken` needs to be set.
            data: SecuritySchemeData::Http {
                scheme: "basic".to_owned(), // `basic`, `digest`, ...
                // Just gives use a hint to the format used
                bearer_format: Some("cm90ZXJrb2hsOkZsb3JpMTIzNA==".to_owned()),
            },
            extensions: Object::default(),
        };
        // Add the requirement for this route/endpoint
        // This can change between routes.
        let mut security_req = SecurityRequirement::new();
        // Each security requirement needs to be met before access is allowed.
        security_req.insert("HttpAuth".to_owned(), Vec::new());
        // These vvvvvvv-----^^^^^^^^ values need to match exactly!
        Ok(RequestHeaderInput::Security(
            "HttpAuth".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}