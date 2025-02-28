use clap::{Parser, Subcommand};

#[derive(Parser)] // Enables automatic CLI parsing
#[command(name = "Task Manager")]
#[command(about = "A simple CLI Task Manager in Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task
    Add { title: String },

    /// List all tasks
    List,

    /// Mark a task as done
    Done { title: String },

    /// Remove a task
    Remove { title: String },
}
