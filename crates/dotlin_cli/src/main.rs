use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dotlin")]
#[command(version)]
#[command(about = "Minimal dotlin CLI placeholder", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Print a greeting
    Hello {
        /// Optional name to greet
        name: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Hello { name }) => {
            let who = name.unwrap_or_else(|| "world".into());
            println!("Hello, {}!", who);
        }
        None => {
            println!("dotlin: a tiny placeholder CLI. Try 'dotlin hello'");
        }
    }
}
