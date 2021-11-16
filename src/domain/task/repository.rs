// use ulid::Ulid;

use super::entity::Task;

pub trait Repository {
    fn create(&self, task: &Task) -> Result<Task, String>;
    // fn find(&self) -> Vec<Task>;
    // fn find_one(&self, id: &Ulid) -> Task;
    // fn update(&self, id: &Ulid, task: &Task) -> Task;
}
