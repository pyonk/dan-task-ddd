use super::dto::{ListTaskDTO, TaskDTO};
use crate::domain::task::entity::Task;
use crate::domain::task::repository::Repository;
use std::{fs, fs::OpenOptions, io::Write};

pub struct TaskRepository {}

impl TaskRepository {
    pub fn new() -> Self {
        Self {}
    }
}

impl Repository for TaskRepository {
    fn create(&self, task: &Task) -> Result<Task, String> {
        let dto = TaskDTO::from(task);
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("task.json")
            .unwrap();
        let result = f.write(serde_json::to_string(&dto).unwrap().as_bytes());
        f.flush().unwrap();
        match result {
            Ok(_) => Ok(Task::from(&dto)),
            Err(err) => Err(format!("Error: {}", err)),
        }
    }

    fn all(&self) -> Vec<Task> {
        let data_from_json: ListTaskDTO =
            serde_json::from_str(&fs::read_to_string("task.json").unwrap()).unwrap();
        data_from_json
            .tasks
            .iter()
            .map(|dto| Task::from(dto))
            .collect()
    }
}
