
use bolt_client::bolt_proto::value::Node;
use chrono::{DateTime, Utc};
use rocket_okapi::okapi::schemars::JsonSchema;

use super::Entity;

#[derive(serde::Serialize, Clone, JsonSchema)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub points: u16,
    pub enabled: bool,
}

impl Entity<u32> for Task {
    fn get_id(&self) -> &u32 {
        &self.id
    }

    fn get_node_type_name() -> &'static str {
        "Task"
    }
}

impl From<Node> for Task {
    fn from(value: Node) -> Self {
        !unimplemented!();
    }
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