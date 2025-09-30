mod flags;
mod task;
mod store;

use anyhow::Result;
use clap::Parser;
use flags::{Cli, Flag};
use store::{load_tasks, save_tasks};
use task::Task;

fn main() -> Result<()>
{
    let cli = Cli::parse();
    let mut tasks = load_tasks()?;

    match cli.command
    {
        Flag::Add {title} =>
        {
            let id = tasks.len() as u32 + 1;
            tasks.push(Task::new(id, title));
            println!("Task has been Added");
        }
        Flag::List =>
        {
            for task in &tasks
            {
                println!("{}: {} [{}]", task.id, task.title, if task.done {"X"} else{" "});
            }
        }
        Flag::Done {id} =>
        {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == id)
            {
                task.done = true;
                println!("Marked Task {} as Completed", id);
            }
            else
            {
                println!("Task {} not found", id);
            }
        }
        Flag::Remove {id} =>
        {
            tasks.retain(|t| t.id != id);
            println!("Removed Task {}", id);
        }
    }
    save_tasks(&tasks)?;
    Ok(())
}
