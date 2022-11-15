
use std::{sync::{Mutex, Arc}};

use rocket::{fairing::Result, http::Status};
use futures::join;

use crate::model::{MessageResponder, Session, Task, User, session::{LoginRequest}, user::{Team}};

use super::repository::Repository;

pub struct LegacyRepository {
    users: Arc<Mutex<Vec<Arc<Mutex<User>>>>>,
    sessions: Arc<Mutex<Vec<Arc<Mutex<Session>>>>>,
    tasks: Arc<Mutex<Vec<Task>>>,
    teams: Arc<Mutex<Vec<Arc<Mutex<Team>>>>>,
}

#[async_trait]
impl Repository for LegacyRepository {

    async fn get_user<'a>(&'a self, id: u32) -> Option<User> {
        match self.get_user_unlocked(id) {
            Some(u) => Some(u.lock().unwrap().clone()),
            None => None
        }
    }
    
    async fn find_user_by_username<'a>(&'a self, username: &String) -> Option<Arc<Mutex<User>>> {
        let users_guard = self.users.lock().unwrap();
        users_guard.iter().find(|user| user.lock().unwrap().username.eq(username)).and_then(|f| Some(f.clone()))
    }
    
    async fn find_user_by_username_const<'a>(&'a self, username: &String) -> Option<User> {
        let users_guard = self.users.lock().unwrap();
        let user = users_guard.iter().find(|user| user.lock().unwrap().username.eq(username)).and_then(|f| Some(f.clone()));
        user.and_then(|u| Some(u.clone().lock().unwrap().clone()))
    }

    async fn get_all_users<'a>(&'a self) -> Vec<User> {
        self.users.lock().unwrap().iter().map(|user_mutex| user_mutex.lock().unwrap().clone()).collect()
    }

    async fn get_task<'a>(&'a self, id: u32) -> Option<Task> {
        self.tasks.lock().unwrap().iter().find(|task| task.id == id).and_then(|temp_task| Some(temp_task.clone()))
    }

    async fn get_all_tasks<'a>(&'a self) -> Vec<Task> {
        self.tasks.lock().unwrap().clone()
    }

    async fn get_session<'a>(&'a self, session_id: &String) -> Option<Session> {
        self.find_session(session_id).and_then(|s| Some(s.lock().unwrap().clone()))
    }

    async fn score<'a>(&'a self, user_id: u32, task_id: u32) -> Result<u16, String> {
        let users_guard = self.users.lock().unwrap();
        let user_opt = users_guard.iter().find(|user| user.lock().unwrap().id == user_id);
        let user_mutex = user_opt.ok_or("User does not exist")?;
        let mut user = user_mutex.lock().unwrap();

        let locked_tasks = self.tasks.lock().unwrap();
        let task_opt = locked_tasks.iter().find(|task| task.id == task_id);
        let task = task_opt.ok_or("Task does not exist")?;

        if !task.enabled {
            return Err("Task is not enabled".to_owned());
        }

        user.score_task(task.clone());

        Ok(user.points)
    }

    async fn create_and_add_user<'a>(&'a self, username: String, display_name: String, password: String, is_admin: bool) -> Result<Arc<Mutex<User>>, String> {
        if self.find_user_by_username(&username).await.is_some() {
            return Err("Username is not available".to_owned());
        }

        let mut user = User::new(0, username, display_name, is_admin);
        user.set_password(password);

        self.add_user_private(user)
    }

    async fn add_team<'a>(&'a self, team: Team) -> Option<u32> {
        let new_team = self.add_team_private(team);
        new_team.and_then(|team| Ok(team.lock().unwrap().id)).ok()
    }

    async fn add_user_to_team<'a>(&'a self, team_name: &String, user_id: u32, manager: User) -> Result<(), String> {
        let teams_locked = self.teams.lock().unwrap();
        let users_locked = self.users.lock().unwrap();

        let team = teams_locked.iter().find(|t| t.lock().unwrap().name == *team_name).ok_or(format!("Team with name '{}'does not exist", team_name))?;
        let user = users_locked.iter().find(|u| u.lock().unwrap().id == user_id).ok_or(format!("User with id {} does not exist", user_id))?;
        let mut team_locked = team.lock().unwrap();
        
        team_locked.add_user(user.clone(), &manager)
    }

    async fn add_user<'a>(&'a self, session: &Session, mut user: User) -> MessageResponder<u32> {
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

    async fn login<'a>(&'a self, login_request: LoginRequest<'a>) -> Result<Session, String> {
        let username = login_request.username.to_owned();
        let user_opt = self.find_user_by_username(&username).await;
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

    async fn logout(&self, session_id: &String) -> Result<(), String> {
        let mut sessions = self.sessions.lock().unwrap();
        let index = sessions.iter().position(|x| &x.lock().unwrap().clone().id == session_id).ok_or("Session unknown".to_owned())?;
        sessions.remove(index);

        Ok(())
    }
}

impl LegacyRepository {

    #[allow(unused)]
    pub async fn init_repository() -> LegacyRepository {

        let users = vec![];

        let sessions = vec![];

        let tasks = vec![
            Task { id: 1, name: "Blumen gießen".to_owned(), points: 10, enabled: true},
            Task { id: 2, name: "Stunden abgeben".to_owned(), points: 30, enabled: false},
            Task { id: 3, name: "Spülmaschine ausräumen".to_owned(), points: 52, enabled: true},
            Task { id: 4, name: "Kaffee kochen".to_owned(), points: 75, enabled: true},
        ];

        let repository = LegacyRepository {
            users: Arc::new(Mutex::new(users)),
            sessions: Arc::new(Mutex::new(sessions)),
            tasks: Arc::new(Mutex::new(tasks)),
            teams: Arc::new(Mutex::new(vec![]))
        };

        let flori = repository.create_and_add_user("roterkohl".to_owned(), "Flori".to_owned(), "Flori1234".to_owned(), true).await.unwrap();
        let michi = repository.create_and_add_user("brutours.de".to_owned(), "Michi".to_owned(), "Michi1234".to_owned(), false).await.unwrap();
        let franki = repository.create_and_add_user("dliwespf".to_owned(), "Franki".to_owned(), "Franki1234".to_owned(), false).await.unwrap();
        let topheri = repository.create_and_add_user("topher".to_owned(), "Topher".to_owned(), "Topheri1234".to_owned(), true).await.unwrap();

        let mut team_babes = Team::new(1, "Babes".to_owned(), flori.clone());
        let mut team_church = Team::new(2, "Church".to_owned(), michi.clone());
        team_church.add_user(franki.clone(), &michi.lock().unwrap());
        team_church.add_user(flori.clone(), &michi.lock().unwrap());
        team_church.add_user(topheri.clone(), &michi.lock().unwrap());

        let flori_id = flori.lock().unwrap().id;
        let michi_id = michi.lock().unwrap().id;
        let franki_id = franki.lock().unwrap().id;
        let topheri_id = topheri.lock().unwrap().id;

        join!(
            repository.score(flori_id, 1),
            repository.score(flori_id, 1),
            repository.score(flori_id, 2),
            repository.score(flori_id, 1),
            repository.score(flori_id, 4),
            repository.score(flori_id, 3),
            repository.score(michi_id, 1),
            repository.score(michi_id, 2),
            repository.score(michi_id, 3),
            repository.score(michi_id, 4),

            repository.score(topheri_id, 4),
            repository.score(topheri_id, 4),
            repository.score(topheri_id, 4),
            repository.score(topheri_id, 4),
            repository.score(topheri_id, 4)
        );

        repository
    }

    fn get_user_unlocked<'a>(&'a self, id: u32) -> Option<Arc<Mutex<User>>> {

        let users_guard = self.users.lock().unwrap();
        users_guard.iter().find(|user| user.lock().unwrap().id == id).and_then(|user| Some(user.clone()))

    }
    
    fn find_session<'a>(&'a self, session_id: &String) -> Option<Arc<Mutex<Session>>> {
        self.sessions.lock().unwrap().iter().find(|session| session.lock().unwrap().id.eq(session_id))
            .and_then(|f| Some(f.clone()))
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

    fn add_team_private<'a>(&'a self, mut team: Team) -> Result<Arc<Mutex<Team>>, String> {
        let mut teams_vec = self.teams.lock().unwrap();

        let new_id = teams_vec.iter().map(|u| u.lock().unwrap().id).max().unwrap_or(0) + 1;
        team.id = new_id;
        let new_team = Arc::new(Mutex::new(team));
        teams_vec.push(new_team.clone());

        Ok(new_team)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use rocket::futures::executor::block_on;

    use crate::model::{Session, session::LoginRequest, User, user::Team};
    use crate::repository::repository::Repository;

    use super::LegacyRepository;

    lazy_static! {
        static ref REPOSITORY: LegacyRepository = block_on(LegacyRepository::init_repository());
        static ref ADMIN_SESSION: Session = block_on(REPOSITORY.login(LoginRequest{username: "roterkohl", password: Some("Flori1234")})).unwrap();
        static ref USER_SESSION: Session = block_on(REPOSITORY.login(LoginRequest{username: "dliwespf", password: Some("Franki1234")})).unwrap();
    }
    
    #[test]
    fn test_get_user_exists() {
        let exists_opt = block_on(REPOSITORY.get_user(1));
        assert!(exists_opt.is_some());
        assert_eq!(1, exists_opt.unwrap().id);
    }

    #[test]
    fn test_get_user_does_not_exist() {
        let does_not_exist_opt = block_on(REPOSITORY.get_user(10));
        assert!(does_not_exist_opt.is_none());
    }

    #[test]
    fn test_find_user_by_username_exists() {
        let result = block_on(REPOSITORY.find_user_by_username(&"topher".to_owned()));
        assert!(result.is_some());
        let user_result = result.unwrap();
        let user = user_result.lock().unwrap();
        assert_eq!(4, user.id);
    }

    #[test]
    fn test_find_user_by_username_does_not_exist() {
        let result = block_on(REPOSITORY.find_user_by_username(&"eduardLaser".to_owned()));
        assert!(result.is_none());
    }

    #[test]
    fn test_get_all_users() {
        let result = block_on(block_on(LegacyRepository::init_repository()).get_all_users());
        assert_eq!(4, result.len());
    }

    #[test]
    fn test_get_task_exists() {
        let exists_opt = block_on(REPOSITORY.get_task(1));
        assert!(exists_opt.is_some());
        assert_eq!(1, exists_opt.unwrap().id);
    }

    #[test]
    fn test_get_task_does_not_exist() {
        let does_not_exist_opt = block_on(REPOSITORY.get_task(4711));
        assert!(does_not_exist_opt.is_none());
    }

    #[test]
    fn test_get_all_tasks() {
        let result = block_on(block_on(LegacyRepository::init_repository()).get_all_tasks());
        assert_eq!(4, result.len());
    }

    #[test]
    fn test_find_session_exists() {
        let result = REPOSITORY.find_session(&ADMIN_SESSION.id);
        assert!(result.is_some());
        assert_eq!(ADMIN_SESSION.id, result.unwrap().lock().unwrap().id);
    }

    #[test]
    fn test_get_session_does_not_exist() {
        let result = REPOSITORY.find_session(&"&ADMIN_SESSION.id".to_owned());
        assert!(result.is_none());
    }

    #[test]
    fn test_score_ok() {
        let result = block_on(REPOSITORY.score(4, 4));
        assert!(result.is_ok());
        assert_eq!(450, result.unwrap());
    }

    #[test]
    fn test_score_user_does_not_exist() {
        let result = block_on(REPOSITORY.score(40, 4));
        assert!(result.is_err());
        assert_eq!("User does not exist", result.unwrap_err());
    }

    #[test]
    fn test_score_task_does_not_exist() {
        let result = block_on(REPOSITORY.score(4, 40));
        assert!(result.is_err());
        assert_eq!("Task does not exist", result.unwrap_err());
    }

    #[test]
    fn test_score_task_is_disabled() {
        let result = block_on(REPOSITORY.score(4, 2));
        assert!(result.is_err());
        assert_eq!("Task is not enabled", result.unwrap_err());
    }

    #[test]
    fn test_create_and_add_user_ok() {
        let result = block_on(REPOSITORY.create_and_add_user("Winston".to_owned(), "Wilson".to_owned(), "fml".to_owned(), false));
        assert!(result.is_ok());
        let new_user_id = result.unwrap().lock().unwrap().id;
        let new_user_result = block_on(REPOSITORY.get_user(new_user_id));
        assert!(new_user_result.is_some());
    }

    #[test]
    fn test_create_and_add_user_name_clash() {
        let result = block_on(REPOSITORY.create_and_add_user("dliwespf".to_owned(), "Wilson".to_owned(), "fml".to_owned(), false));
        let err = result.err();
        
        assert!(err.is_some());
        assert_eq!("Username is not available", err.unwrap());
    }

    #[test]
    fn test_add_team() {
        let repository = block_on(LegacyRepository::init_repository());
        let new_id = repository.add_team(Team::new(0, "newTeam".to_owned(), repository.get_user_unlocked(3).unwrap().clone()));
        //let team = repository.get_
    }

    #[test]
    fn test_add_user_to_team() {
        //REPOSITORY.add_user_to_team(team_name, user_id, manager)
    }

    #[test]
    fn test_add_user() {
        let newbie = User::new(0, "newbie".to_owned(), "Newbie".to_owned(), false);
        let result = block_on(REPOSITORY.add_user(&ADMIN_SESSION, newbie)).content;
        let new_user_id = result.unwrap();
        assert!(result.is_some());
        let new_user_result = block_on(REPOSITORY.get_user(new_user_id));
        assert!(new_user_result.is_some());
    }

    #[test]
    fn test_login_ok() {
        let login_request = LoginRequest { username: "roterkohl", password: Some("Flori1234") };
        let result = block_on(REPOSITORY.login(login_request));
        assert!(result.is_ok());
        assert_eq!(30, result.unwrap().id.len());
    }

    #[test]
    fn test_login_missing_user() {
        let login_request = LoginRequest { username: "blauerkohl", password: Some("Flori1234") };
        let result = block_on(REPOSITORY.login(login_request));
        assert!(result.is_err());
        let err = result.err();
        assert_eq!("User does not exist", err.unwrap());
    }

    #[test]
    fn test_login_wrong_password() {
        let login_request = LoginRequest { username: "roterkohl", password: Some("Flori5678") };
        let result = block_on(REPOSITORY.login(login_request));
        assert!(result.is_err());
        let err = result.err();
        assert_eq!("Password mismatch", err.unwrap());
    }

    #[test]
    fn test_logout_ok() {
        let result = block_on(REPOSITORY.logout(&USER_SESSION.id));
        assert!(result.is_ok())
    }

    #[test]
    fn test_logout_not_logged_in() {
        let result = block_on(REPOSITORY.logout(&"&USER_SESSION.id".to_owned()));
        assert!(result.is_err());
        assert_eq!("Session unknown", result.unwrap_err())
    }
}