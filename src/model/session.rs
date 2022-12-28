use std::{sync::{Arc, Mutex}, fmt::Display};

use bolt_client::bolt_proto::value::Node;
use chrono::{DateTime, Utc};
use okapi::openapi3::{Object, SecurityScheme, SecuritySchemeData, SecurityRequirement};
use rocket::{Request, http::Status, request::FromRequest, request::Outcome};
use rocket_okapi::{request::{OpenApiFromRequest, RequestHeaderInput}, gen::OpenApiGenerator};
use schemars::{JsonSchema};
use base64;

use crate::logic::logic::{Logic, ApplicationLogic};

use super::{User, util::{get_string, get_utc}};
use super::entity::{Entity, FromInput};
use rand::Rng;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const PASSWORD_LEN: usize = 30;

#[derive(serde::Serialize, Clone, OpenApiFromRequest, JsonSchema, Debug)]
pub struct Session {
    pub id: Option<u32>,
    pub session_id: String,
    pub user: Arc<Mutex<User>>,
    pub started: DateTime::<Utc>,
    pub refreshed: DateTime::<Utc>,
}

impl Session {
    pub fn new(id: Option<u32>, user: Arc<Mutex<User>>) -> Session {
        let now = Utc::now();
        Session {
            id,
            session_id: Session::generate_session_id(),
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

impl Entity for Session {
    type I = u32;

    fn get_id(&self) -> &Option<u32>{
        &self.id
    }

    fn get_node_type_name() -> &'static str {
        "Session"
    }
}

impl Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_entity(f)
    }
}

impl TryFrom<FromInput> for Session {
    type Error = String;
    fn try_from(input: FromInput) -> Result<Self, Self::Error> {
        let mut node_map = input.0;
        let session_node_opt = node_map.remove(Session::get_node_type_name());
        let user_node_opt = node_map.remove(User::get_node_type_name());

        if session_node_opt.is_none() {
            return Err(String::from("Unable to create session from db node; no session nodes available"))
        }

        let mut session_node_vec = session_node_opt.unwrap();

        if session_node_vec.len() != 1 {
            return Err(format!("Unable to create session from db node; unusual number of session nodes: {}", session_node_vec.len()));
        }

        let session_node = session_node_vec.pop().unwrap();
        let mut session = Session::from(session_node);

        if user_node_opt.is_some() {
            let mut user_node_vec = user_node_opt.unwrap();

            if user_node_vec.len() != 1 {
                println!("Error while creating session from db node; unusual number of user nodes: {}", user_node_vec.len());
            }

            let user_node = user_node_vec.pop().unwrap();
            let user = User::from(user_node);
            session.user = Arc::new(Mutex::new(user));
        }

        Ok(session)
    }
}

impl From<Node> for Session {
    fn from(value: Node) -> Self {
        let properties = value.properties();
        let id =  Some(value.node_identity() as u32);
        let session_id =  get_string(properties, "session_id", "");
        let started = get_utc(properties, "started", Utc::now());
        let refreshed = get_utc(properties, "refreshed", Utc::now());
        let session = Session {id, session_id, user: Arc::new(Mutex::new(User::get_default_user())), started, refreshed};
        session
    }
}

#[async_trait]
impl <'a> FromRequest<'a> for Session {
    type Error = String;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let repository = request.rocket().state::<ApplicationLogic>();
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

                let decoded_res = base64::decode_engine(login_data_encoded, &base64::engine::DEFAULT_ENGINE);
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