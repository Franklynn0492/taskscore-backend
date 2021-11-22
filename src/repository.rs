use crate::model::{tasks::*, users::{*, self}};
use std::sync::Mutex;

pub struct Repository {
    users: Box<Vec<Mutex<User>>>,
    tasks: Box<Vec<Task>>
}

impl Repository {
    pub fn init_repository() -> Repository {
        let mut flori = User::new(1, "Flori");
        let mut michi = User::new(2, "Michi");
        let franki = User::new(3, "Franki");

        let tasks = vec![
            Task { id: 1, name: "Blumen gießen".to_owned(), points: 10, enabled: true},
            Task { id: 2, name: "Stunden abgeben".to_owned(), points: 30, enabled: false},
            Task { id: 3, name: "Spülmaschine ausräumen".to_owned(), points: 52, enabled: true},
            Task { id: 4, name: "Kaffee kochen".to_owned(), points: 75, enabled: true},
        ];

        flori.score_task(tasks[0].clone());
        flori.score_task(tasks[0].clone());
        flori.score_task(tasks[1].clone());
        flori.score_task(tasks[0].clone());
        flori.score_task(tasks[3].clone());
        flori.score_task(tasks[2].clone());
        
        michi.score_task(tasks[0].clone());
        michi.score_task(tasks[1].clone());
        michi.score_task(tasks[2].clone());
        michi.score_task(tasks[3].clone());

        let mut users = vec![
            Mutex::new(flori),
            Mutex::new(michi),
            Mutex::new(franki)
        ];

        Repository {users: Box::new(users), tasks: Box::new(tasks)}
    }

    pub fn get_user<'a>(&'a self, id: u32) -> Option<User> {
        let userMutex = self.users.iter().find(|user| user.lock().unwrap().id == id);
        match userMutex {
            Some(userMut) => Some(userMut.lock().unwrap().clone()),
            _ => None
        }
    }

    pub fn get_all_users<'a>(&'a self) -> Vec<User> {
        self.users.iter().map(|user_mutex| user_mutex.lock().unwrap().clone()).collect()
    }

    pub fn get_task<'a>(&'a self, id: u32) -> Option<&'a Task> {
        self.tasks.iter().find(|task| task.id == id)
    }

    pub fn get_all_tasks<'a>(&'a self) -> &'a Vec<Task> {
        &self.tasks
    }

    pub fn score<'a>(&'a self, user_id: u32, task_id: u32) -> Result<u16, String> {
        let user_opt = self.users.iter().find(|user| user.lock().unwrap().id == user_id);
        let user_mutex = user_opt.ok_or("User does not exist")?;
        let mut user = user_mutex.lock().unwrap();

        let task_opt = self.tasks.iter().find(|task| task.id == task_id);
        let task = task_opt.ok_or("Task does not exist")?;

        user.score_task(task.clone());

        Ok(user.points)
    }
}