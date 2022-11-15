
use chrono::{DateTime, Utc};
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(serde::Serialize, Clone, JsonSchema)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub points: u16,
    pub enabled: bool,
}

#[derive(serde::Serialize, Clone, JsonSchema)]
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