
use std::fmt::Display;

use bolt_client::bolt_proto::value::Node;
use chrono::{DateTime, Utc};
use rocket_okapi::okapi::schemars::JsonSchema;

use super::Entity;

#[derive(serde::Serialize, Clone, JsonSchema, Debug)]
pub struct Task {
    pub id: Option<u32>,
    pub name: String,
    pub points: u16,
    pub enabled: bool,
}

impl Entity for Task {
    type I = u32;

    fn get_id(&self) -> Option<&u32>{
        self.id.and_then(|i| Some(&i))
    }

    fn get_node_type_name() -> &'static str {
        "Task"
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_entity(f)
    }
}

impl From<Node> for Task {
    fn from(value: Node) -> Self {
        !unimplemented!();
    }
}

#[derive(serde::Serialize, Clone, JsonSchema, Debug)]
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