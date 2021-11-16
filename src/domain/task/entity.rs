use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use ulid::Ulid;

pub struct Task {
    pub id: Ulid,
    pub name: String,
    pub status: TaskStatus,
}

#[derive(Debug)]
pub enum TaskStatus {
    ToDo,
    Doing,
    Done,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl FromStr for TaskStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ToDo" => Ok(Self::ToDo),
            "Doing" => Ok(Self::Doing),
            "Done" => Ok(Self::Done),
            _ => Err(()),
        }
    }
}
