use std::io::Error;
use clap::Args;
use crate::commands::Command;

#[derive(Debug, Args)]
pub struct InitCommand {
    #[arg(short, long = "text")]
    pub text: String,
}

impl Command for InitCommand {
    const NAME: &'static str = "init";
    fn run(&self) -> Result<(), Box<Error>> {
        println!("InitCommand, {}", self.text);
        Ok(())
    }
}