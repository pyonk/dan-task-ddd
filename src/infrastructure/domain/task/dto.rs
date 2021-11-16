use std::{fs::OpenOptions, io::Write, str::FromStr};

use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::domain::task::entity::{Task, TaskStatus};

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

#[derive(Serialize, Deserialize)]
pub struct TaskDTO {
    id: String,
    name: String,
    status: String,
}

impl TaskDTO {
    pub fn from(task: &Task) -> Self {
        Self {
            id: task.id.to_string(),
            name: task.name.to_string(),
            status: task.status.to_string(),
        }
    }

    pub fn save(&self) -> Result<&TaskDTO, std::io::Error> {
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("task.json")
            .unwrap();
        let result = f.write(serde_json::to_string(self).unwrap().as_bytes());
        f.flush().unwrap();
        match result {
            Ok(_) => Ok(self),
            Err(err) => Err(err),
        }
    }
}

impl Task {
    pub fn from(dto: &TaskDTO) -> Self {
        Self {
            id: Ulid::from_string(&dto.id).unwrap(),
            name: dto.name.to_string(),
            status: TaskStatus::from_str(&dto.status).unwrap(),
        }
    }
}
