use crate::task::Task;
use anyhow::Result;
use std::fs;
use std::path::Path;

const FILE: &str = "tasks.json";

pub fn load_tasks() -> Result<Vec<Task>>
{
    if !Path::new(FILE).exists()
    {
        return Ok(Vec::new());
    }
    let data = fs::read_to_string(FILE)?;
    let tasks: Vec<Task> = serde_json::from_str(&data)?;
    Ok(tasks)
}

pub fn save_tasks(tasks: &Vec<Task>) -> Result<()>
{
    let data = serde_json::to_string_pretty(tasks)?;
    fs::write(FILE, data)?;
    Ok(())
}
