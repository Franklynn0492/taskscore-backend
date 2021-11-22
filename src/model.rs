

pub mod users {
    #[derive(serde::Serialize)]
    pub struct User {
        pub id: u32,
        pub name: String,
    }
}

pub mod tasks {
    use chrono::{DateTime, Utc};

    #[derive(serde::Serialize)]
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
}