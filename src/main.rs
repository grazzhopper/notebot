use clap::{Parser, Subcommand};
mod config;
mod task;

#[derive(Parser)]
#[command(name = "notebot")]
#[command(about = "A CLI assistant for task management")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Placeholder for task commands
    Task,
}

fn main() -> anyhow::Result<()> {
    config::init_storage()?;
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Task) => println!("Task command placeholder"),
        None => println!("No command provided"),
    }
    Ok(())
}
