use clap::{
    Parser, 
    Subcommand
};
use commands::{
    Command,
    clone::InitCommand
};
mod commands;

#[derive(Debug, Parser)]
#[command(
    name = "gitrs",
    about = "A Git implementation in Rust.",
    disable_help_flag = true
)]
struct Git {
    #[command(subcommand)]
    sub: Option<Subcommands>,

    #[arg(short, long)]
    test: bool,
}

#[derive(Debug, Subcommand)]
enum Subcommands {
    Init(InitCommand)
}

pub fn main() {
    let git = Git::parse();

    if git.test {
        println!("Test...");
    }

    if let Some(subcommand) = git.sub {
        match subcommand {
            Subcommands::Init(cmd) => {
               let _ = cmd.run();
            }
        }
    }
}