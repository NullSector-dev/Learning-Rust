use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo", about = "A To-Do CLI App", long_about = None)]
pub struct Cli
{
    #[command(subcommand)]
    pub command: Flag,
}

#[derive(Subcommand, Debug)]
pub enum Flag
{
    #[command(about = "Add a new Task")]
    Add{#[arg(help = "The title of the Task")] title: String},
    #[command(about = "List all tasks")]
    List,
    #[command(about = "Mark a Task as completed")]
    Done{#[arg(help = "Provide ID of the Task to be marked complete")] id: u32},
    #[command(about = "Remove a Task")]
    Remove{#[arg(help = "Provide ID of the Task to be removed")] id: u32},
}
