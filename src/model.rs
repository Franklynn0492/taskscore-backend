

pub mod users {
    use std::convert::TryFrom;

    use super::tasks::{self, Score, Task};
    use bcrypt::{DEFAULT_COST, hash, verify};
    use rocket::{Request, request::Outcome, http::Status, request::{self, FromRequest}};

    #[derive(serde::Serialize)]
    #[derive(Clone)]
    pub struct User {
        pub id: u32,
        pub username: String,
        pub display_name: String,
        pub points: u16,
        pub scores: Vec<tasks::Score>,
        
        #[serde(skip_serializing)]
        pwd_hash_components: Option<String>,
    }

    impl User {
        pub fn new(id: u32, username: String, display_name: String) -> User {
            User {id, username: username, display_name: display_name, points: 0, scores: vec![], pwd_hash_components: None}
        }

        pub fn score_task<'a>(& mut self, task: Task) {
            self.points += task.points;

            let score = Score::new(task);
            self.scores.push(score);
        }

        pub fn set_password(&mut self, password: String) {
            self.pwd_hash_components = Some(bcrypt::hash(password, DEFAULT_COST).unwrap());
        }

        pub fn verify_password(&self, password_to_verify: String) -> bool {
            match &self.pwd_hash_components {
                Some(hash) => bcrypt::verify(password_to_verify, hash.as_str()).unwrap(),
                None => true
            }
        }
    }

    #[async_trait]
    impl <'a> FromRequest<'a>  for User {
        type Error = String;

        async fn from_request(request: &'a Request<'_>) -> request::Outcome<Self, Self::Error> {
            let username_opt = request.headers().get_one("username");
            let display_name_opt = request.headers().get_one("display_name");
            let password_opt = request.headers().get_one("password");
            match username_opt {
                Some(username) => {
                    let mut new_user = User::new(0, username.to_owned(), display_name_opt.unwrap_or(username).to_owned());
                    if password_opt.is_some() {
                        new_user.set_password(password_opt.unwrap().to_owned());
                    }

                    Outcome::Success(new_user)
                },
                None => Outcome::Failure((Status::BadRequest, "Username is required".to_owned()))
            }
        }
    }
}

pub mod tasks {
    use chrono::{DateTime, Utc};

    #[derive(serde::Serialize)]
    #[derive(Clone)]
    pub struct Task {
        pub id: u32,
        pub name: String,
        pub points: u16,
        pub enabled: bool,
    }

    #[derive(serde::Serialize)]
    #[derive(Clone)]
    pub struct Score {
        pub task: Task,
        pub points: u16,
        pub scored_at: DateTime::<Utc>,
    }

    impl Score {
        pub fn new(task: Task) -> Score {
            let points = task.points.clone();
            Score { task, points, scored_at: chrono::Utc::now()}
        }
    }
}