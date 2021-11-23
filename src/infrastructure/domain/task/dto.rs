use std::str::FromStr;

use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::domain::task::entity::{Task, TaskStatus};

#[derive(Serialize, Deserialize)]
pub struct ListTaskDTO {
    pub tasks: Vec<TaskDTO>,
}
pub struct NewTaskDTO {
    name: String,
}

impl NewTaskDTO {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl TryFrom<NewTaskDTO> for Task {
    type Error = ();

    fn try_from(dto: NewTaskDTO) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Ulid::new(),
            name: dto.name.to_string(),
            status: TaskStatus::ToDo,
        })
    }
}

impl From<&TaskDTO> for Task {
    fn from(dto: &TaskDTO) -> Self {
        Self {
            id: Ulid::from_str(&dto.id.to_string()).unwrap(),
            name: dto.name.to_string(),
            status: TaskStatus::from_str(&dto.status).unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TaskDTO {
    id: String,
    name: String,
    status: String,
}

impl From<&Task> for TaskDTO {
    fn from(task: &Task) -> Self {
        Self {
            id: task.id.to_string(),
            name: task.name.to_string(),
            status: task.status.to_string(),
        }
    }
}
