

pub mod users {
    use super::tasks::{self, Score, Task};

    #[derive(serde::Serialize)]
    pub struct User {
        pub id: u32,
        pub name: String,
        pub points: u16,
        pub scores: Vec<tasks::Score>,
    }

    impl User {
        pub fn new(id: u32, name: &str) -> User {
            User {id, name: name.to_owned(), points: 0, scores: vec![]}
        }

        pub fn score_task<'a>(& mut self, task: Task) {
            self.points += task.points;

            let score = Score::new(task);
            self.scores.push(score);
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