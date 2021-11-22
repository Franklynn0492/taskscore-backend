use crate::model::{tasks::*, users::{*, self}};

pub struct Repository {
    users: Vec<User>,
    tasks: Vec<Task>
}

impl Repository {
    pub fn init_repository() -> Repository {
        let users = vec![
            User { id: 1, name: "Flori".to_owned()},
            User { id: 2, name: "Michi".to_owned()},
            User { id: 3, name: "Franki".to_owned()}
        ];

        let tasks = vec![
            Task { id: 1, name: "Blumen gießen".to_owned(), points: 10, enabled: true},
            Task { id: 2, name: "Stunden abgeben".to_owned(), points: 30, enabled: false},
            Task { id: 3, name: "Kaffee kochen".to_owned(), points: 75, enabled: true},
        ];

        Repository {users, tasks}
    }

    pub fn get_user<'a>(&'a self, id: u32) -> Option<&'a User> {
        self.users.iter().find(|user| user.id == id)
    }
}