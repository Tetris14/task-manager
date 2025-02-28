mod cli;
mod commands;
mod connect;

use clap::Parser;
use cli::{Cli, Commands};
use commands::add::add_task;
use commands::complete::complete_task;
use commands::list::list_tasks;
use commands::remove::remove_task;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { title } => add_task(title),
        Commands::List => list_tasks(),
        Commands::Done { title } => complete_task(title),
        Commands::Remove { title } => remove_task(title),
    }
}
