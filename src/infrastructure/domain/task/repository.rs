use super::dto::TaskDTO;
use crate::domain::task::entity::Task;
use crate::domain::task::repository::Repository;

pub struct TaskRepository {}

impl TaskRepository {
    pub fn new() -> Self {
        Self {}
    }
}

impl Repository for TaskRepository {
    fn create(&self, task: &Task) -> Result<Task, String> {
        let dto = TaskDTO::from(task);
        let result = dto.save();
        match result {
            Ok(_) => Ok(Task::from(result.unwrap())),
            Err(err) => Err(format!("Error: {}", err)),
        }
    }
}
