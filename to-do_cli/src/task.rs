use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Task
{
    pub id: u32,
    pub title: String,
    pub done: bool,
}

impl Task
{
    pub fn new(id: u32, title: String) -> Self
    {
        Self {id, title, done: false}
    }
}
