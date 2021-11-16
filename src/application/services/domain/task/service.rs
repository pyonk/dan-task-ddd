use crate::{
    domain::task::{entity::Task, repository::Repository as TaskRepository},
    infrastructure::domain::task::dto::NewTaskDTO,
};
pub struct TaskService<R>
where
    R: TaskRepository,
{
    task_repository: R,
}

impl<R> TaskService<R>
where
    R: TaskRepository,
{
    pub fn new(task_repository: R) -> Self {
        Self { task_repository }
    }

    pub fn create_task(&self, dto: NewTaskDTO) -> Result<Task, String> {
        let task = Task::try_from(dto).unwrap();
        self.task_repository.create(&task)
    }
}
