use crate::model::{tasks::*, users::{*, self}};
use std::{ops::Add, sync::{Mutex, Arc}};

pub struct Repository {
    users: Arc<Mutex<Vec<Arc<Mutex<User>>>>>,
    tasks: Box<Vec<Task>>
}

impl Repository {
    pub fn init_repository() -> Repository {

        let mut users = vec![];

        let tasks = vec![
            Task { id: 1, name: "Blumen gießen".to_owned(), points: 10, enabled: true},
            Task { id: 2, name: "Stunden abgeben".to_owned(), points: 30, enabled: false},
            Task { id: 3, name: "Spülmaschine ausräumen".to_owned(), points: 52, enabled: true},
            Task { id: 4, name: "Kaffee kochen".to_owned(), points: 75, enabled: true},
        ];

        let mut repository = Repository {users: Arc::new(Mutex::new(users)), tasks: Box::new(tasks)};

        let flori_id = repository.create_and_add_user("roterkohl".to_owned(), "Flori".to_owned(), "Flori1234".to_owned()).unwrap();
        let michi_id = repository.create_and_add_user("brutours.de".to_owned(), "Michi".to_owned(), "Michi1234".to_owned()).unwrap();
        let franki_id = repository.create_and_add_user("dliwespf".to_owned(), "Franki".to_owned(), "Franki1234".to_owned()).unwrap();

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

        repository
    }

    pub fn get_user<'a>(&'a self, id: u32) -> Option<User> {
        let users_guard = self.users.lock().unwrap();
        let userMutex = users_guard.iter().find(|user| user.lock().unwrap().id == id);
        match userMutex {
            Some(userMut) => Some(userMut.lock().unwrap().clone()),
            _ => None
        }
    }

    pub fn get_all_users<'a>(&'a self) -> Vec<User> {
        self.users.lock().unwrap().iter().map(|user_mutex| user_mutex.lock().unwrap().clone()).collect()
    }

    pub fn get_task<'a>(&'a self, id: u32) -> Option<&'a Task> {
        self.tasks.iter().find(|task| task.id == id)
    }

    pub fn get_all_tasks<'a>(&'a self) -> &'a Vec<Task> {
        &self.tasks
    }

    pub fn score<'a>(&'a self, user_id: u32, task_id: u32) -> Result<u16, String> {
        let users_guard = self.users.lock().unwrap();
        let user_opt = users_guard.iter().find(|user| user.lock().unwrap().id == user_id);
        let user_mutex = user_opt.ok_or("User does not exist")?;
        let mut user = user_mutex.lock().unwrap();

        let task_opt = self.tasks.iter().find(|task| task.id == task_id);
        let task = task_opt.ok_or("Task does not exist")?;

        user.score_task(task.clone());

        Ok(user.points)
    }

    pub fn create_and_add_user<'a>(&'a self, username: String, display_name: String, password: String) -> Result<u32, String> {

        let mut user = User::new(0, username, display_name);
        user.set_password(password);

        self.add_user(user)
    }

    pub fn add_user<'a>(&'a self, mut user: User) -> Result<u32, String> {
        let mut users_vec = self.users.lock().unwrap();

        if users_vec.iter().any(|u| u.lock().unwrap().username.eq(&user.username)) {
            return Err("Username is not available".to_owned());
        }

        let new_id = users_vec.iter().map(|u| u.lock().unwrap().id).max().unwrap_or(0) + 1;
        user.id = new_id;
        users_vec.push(Arc::new(Mutex::new(user)));

        Ok(new_id)
    }
}