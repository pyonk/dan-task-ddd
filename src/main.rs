mod application;
mod cli;
mod domain;
mod infrastructure;
use application::services::domain::task::service::TaskService;
use cli::{Cli, Command};
use infrastructure::domain::task::{dto::NewTaskDTO, repository::TaskRepository};
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();
    let task_service = TaskService::new(TaskRepository::new());
    match args.command {
        Some(cmd) => match cmd {
            Command::Add { arg } => match task_service.create_task(NewTaskDTO::new(arg)) {
                Ok(task) => {
                    println!("Created! {}", task.name)
                }
                Err(err) => {
                    panic!("{}", err)
                }
            },
        },
        None => {
            println!("No Command")
        }
    }
}
