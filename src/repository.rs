use crate::model::{tasks::*, users::{*, self}};

pub struct Repository {
    users: Box<Vec<User>>,
    tasks: Box<Vec<Task>>
}

impl Repository {
    pub fn init_repository() -> Repository {
        let mut users = vec![
            User::new(1, "Flori"),
            User::new(2, "Michi"),
            User::new(3, "Franki")
        ];

        let tasks = vec![
            Task { id: 1, name: "Blumen gießen".to_owned(), points: 10, enabled: true},
            Task { id: 2, name: "Stunden abgeben".to_owned(), points: 30, enabled: false},
            Task { id: 2, name: "Spülmaschine ausräumen".to_owned(), points: 52, enabled: true},
            Task { id: 3, name: "Kaffee kochen".to_owned(), points: 75, enabled: true},
        ];

        users[0].score_task(tasks[0].clone());
        users[0].score_task(tasks[0].clone());
        users[0].score_task(tasks[1].clone());
        users[0].score_task(tasks[0].clone());
        users[0].score_task(tasks[3].clone());
        users[0].score_task(tasks[2].clone());
        
        users[1].score_task(tasks[1].clone());
        users[1].score_task(tasks[2].clone());
        users[1].score_task(tasks[3].clone());

        Repository {users: Box::new(users), tasks: Box::new(tasks)}
    }

    pub fn get_user<'a>(&'a self, id: u32) -> Option<&'a User> {
        self.users.iter().find(|user| user.id == id)
    }

    pub fn get_all_users<'a>(&'a self) -> &'a Vec<User> {
        &self.users
    }

    pub fn get_task<'a>(&'a self, id: u32) -> Option<&'a Task> {
        self.tasks.iter().find(|task| task.id == id)
    }

    pub fn get_all_tasks<'a>(&'a self) -> &'a Vec<Task> {
        &self.tasks
    }
}