
use std::{ops::Add, sync::{Mutex, Arc}, fmt};

use rocket::{data::Outcome, fairing::Result, http::Status};

use crate::model::{MessageResponder, Session, Task, User, session::{self, LoginRequest}, user::{self, Team}};

pub struct Repository {
    users: Arc<Mutex<Vec<Arc<Mutex<User>>>>>,
    sessions: Arc<Mutex<Vec<Arc<Mutex<Session>>>>>,
    tasks: Arc<Mutex<Vec<Task>>>,
    teams: Arc<Mutex<Vec<Arc<Mutex<Team>>>>>,
}

impl Repository {
    pub fn init_repository() -> Repository {

        let mut users = vec![];

        let mut sessions = vec![];

        let tasks = vec![
            Task { id: 1, name: "Blumen gießen".to_owned(), points: 10, enabled: true},
            Task { id: 2, name: "Stunden abgeben".to_owned(), points: 30, enabled: false},
            Task { id: 3, name: "Spülmaschine ausräumen".to_owned(), points: 52, enabled: true},
            Task { id: 4, name: "Kaffee kochen".to_owned(), points: 75, enabled: true},
        ];

        let mut repository = Repository {
            users: Arc::new(Mutex::new(users)),
            sessions: Arc::new(Mutex::new(sessions)),
            tasks: Arc::new(Mutex::new(tasks)),
            teams: Arc::new(Mutex::new(vec![]))
        };

        let flori = repository.create_and_add_user("roterkohl".to_owned(), "Flori".to_owned(), "Flori1234".to_owned(), true).unwrap();
        let michi = repository.create_and_add_user("brutours.de".to_owned(), "Michi".to_owned(), "Michi1234".to_owned(), false).unwrap();
        let franki = repository.create_and_add_user("dliwespf".to_owned(), "Franki".to_owned(), "Franki1234".to_owned(), false).unwrap();
        let topheri = repository.create_and_add_user("topher".to_owned(), "Topher".to_owned(), "Topheri1234".to_owned(), true).unwrap();

        let mut team_babes = Team::new("Babes".to_owned(), flori.clone());
        let mut team_church = Team::new("Church".to_owned(), michi.clone());
        team_church.add_user(franki.clone(), &michi.lock().unwrap());
        team_church.add_user(flori.clone(), &michi.lock().unwrap());
        team_church.add_user(topheri.clone(), &michi.lock().unwrap());

        let flori_id = flori.lock().unwrap().id;
        let michi_id = michi.lock().unwrap().id;
        let franki_id = franki.lock().unwrap().id;
        let topheri_id = topheri.lock().unwrap().id;

        repository.score(flori_id, 1);
        repository.score(flori_id, 1);
        repository.score(flori_id, 2);
        repository.score(flori_id, 1);
        repository.score(flori_id, 4);
        repository.score(flori_id, 3);

        repository.score(michi_id, 1);
        repository.score(michi_id, 2);
        repository.score(michi_id, 3);
        repository.score(michi_id, 4);

        repository.score(topheri_id, 4);
        repository.score(topheri_id, 4);
        repository.score(topheri_id, 4);
        repository.score(topheri_id, 4);
        repository.score(topheri_id, 4);

        repository
    }

    pub fn get_user<'a>(&'a self, id: u32) -> Option<User> {
        let users_guard = self.users.lock().unwrap();
        users_guard.iter().find(|user| user.lock().unwrap().id == id).and_then(|u| Some(u.lock().unwrap().clone()))
    }
    
    fn find_user_by_username<'a>(&'a self, username: &String) -> Option<Arc<Mutex<User>>> {
        let users_guard = self.users.lock().unwrap();
        users_guard.iter().find(|user| user.lock().unwrap().username.eq(username)).and_then(|f| Some(f.clone()))
    }

    pub fn get_all_users<'a>(&'a self) -> Vec<User> {
        self.users.lock().unwrap().iter().map(|user_mutex| user_mutex.lock().unwrap().clone()).collect()
    }

    pub fn get_task<'a>(&'a self, id: u32) -> Option<Task> {
        self.tasks.lock().unwrap().iter().find(|task| task.id == id).and_then(|temp_task| Some(temp_task.clone()))
    }

    pub fn get_all_tasks<'a>(&'a self) -> Vec<Task> {
        self.tasks.lock().unwrap().clone()
    }
    
    fn find_session<'a>(&'a self, session_id: &String) -> Option<Arc<Mutex<Session>>> {
        self.sessions.lock().unwrap().iter().find(|session| session.lock().unwrap().id.eq(session_id))
            .and_then(|f| Some(f.clone()))
    }

    pub fn get_session<'a>(&'a self, session_id: &String) -> Option<Session> {
        self.find_session(session_id).and_then(|s| Some(s.lock().unwrap().clone()))
    }

    pub fn score<'a>(&'a self, user_id: u32, task_id: u32) -> Result<u16, String> {
        let users_guard = self.users.lock().unwrap();
        let user_opt = users_guard.iter().find(|user| user.lock().unwrap().id == user_id);
        let user_mutex = user_opt.ok_or("User does not exist")?;
        let mut user = user_mutex.lock().unwrap();

        let locked_tasks = self.tasks.lock().unwrap();
        let task_opt = locked_tasks.iter().find(|task| task.id == task_id);
        let task = task_opt.ok_or("Task does not exist")?;

        user.score_task(task.clone());

        Ok(user.points)
    }

    pub fn create_and_add_user<'a>(&'a self, username: String, display_name: String, password: String, is_admin: bool) -> Result<Arc<Mutex<User>>, String> {
        if self.find_user_by_username(&username).is_some() {
            return Err("Username is not available".to_owned());
        }

        let mut user = User::new(0, username, display_name, is_admin);
        user.set_password(password);

        self.add_user_private(user)
    }

    pub fn add_team<'a>(&'a self, team: Team) {
        self.teams.lock().unwrap().push(Arc::new(Mutex::new(team)));
    }

    pub fn add_user_to_team<'a>(&'a self, team_name: &String, user_id: u32, manager: User) -> Result<(), String> {
        let teams_locked = self.teams.lock().unwrap();
        let users_locked = self.users.lock().unwrap();

        let team = teams_locked.iter().find(|t| t.lock().unwrap().name == *team_name).ok_or(format!("Team with name '{}'does not exist", team_name))?;
        let user = users_locked.iter().find(|u| u.lock().unwrap().id == user_id).ok_or(format!("User with id {} does not exist", user_id))?;
        let mut team_locked = team.lock().unwrap();
        
        team_locked.add_user(user.clone(), &manager)
    }

    pub fn add_user<'a>(&'a self, session: &Session, mut user: User) -> MessageResponder<u32> {
        let user_mutex_guard = session.user.lock().unwrap();
        if !user_mutex_guard.is_admin {
            MessageResponder::create_with_message(Status::Forbidden, "You are not an admin".to_owned())
        } else {
            drop(user_mutex_guard);
            match self.add_user_private(user) {
                Ok(user) => MessageResponder::create_ok(user.lock().unwrap().id),
                Err(text) => MessageResponder::create_with_message(Status::Conflict, text)
            }
        }
    }

    fn add_user_private<'a>(&'a self, mut user: User) -> Result<Arc<Mutex<User>>, String> {
        let mut users_vec = self.users.lock().unwrap();

        if users_vec.iter().any(|u| u.lock().unwrap().username.eq(&user.username)) {
            return Err("Username is not available".to_owned());
        }

        let new_id = users_vec.iter().map(|u| u.lock().unwrap().id).max().unwrap_or(0) + 1;
        user.id = new_id;
        let new_user = Arc::new(Mutex::new(user));
        users_vec.push(new_user.clone());

        Ok(new_user)
    }

    pub fn login(&self, login_request: LoginRequest) -> Result<Session, String> {
        let username = login_request.username.to_owned();
        let user_opt = self.find_user_by_username(&username);
        let user = user_opt.ok_or("User does not exist")?;

        if user.lock().unwrap().verify_password(login_request.password) {
            let session = Session::new(Arc::clone(&user));

            let mut sessions = self.sessions.lock().unwrap();
            sessions.push(Arc::new(Mutex::new(session.clone())));

            Ok(session)
        } else {
            Err("Password mismatch".to_owned())
        }
    }

    pub fn logout(&self, session_id: &String) -> Result<(), String> {
        let mut sessions = self.sessions.lock().unwrap();
        let index = sessions.iter().position(|x| &x.lock().unwrap().clone().id == session_id)
        .ok_or("Session unknown".to_owned())?;
        sessions.remove(index);

        Ok(())
    }
}