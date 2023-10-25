// task_team



use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::app::list::Team;

#[derive(Clone, PartialEq, Properties,Serialize,Deserialize)]
pub struct TaskRepository {
    tasks: Vec<Team>,
    next_id: u32,
}

impl TaskRepository {
    pub fn new() -> Self {
        TaskRepository {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_task(&mut self, description: String) -> &Team {
        let new_task = Team {
            team: description.to_string(),
            poule: 0,
            is_removed: false,
        };
        self.tasks.push(new_task);
        self.next_id += 1;
        &self.tasks[self.tasks.len() - 1] // Return a reference to the added task
    }

    pub fn remove_task(&mut self, description: String) -> bool {
        if let Some(index) = self.tasks.iter().position(|task| task.team == description) {
            self.tasks.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_tasks(&self) -> &Vec<Team> {
        &self.tasks
    }
}
