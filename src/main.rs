use clap::{Parser, Subcommand};

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

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Task) => println!("Task command placeholder"),
        None => println!("No command provided"),
    }
}
