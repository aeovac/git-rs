use std::io::Error;

pub(crate) mod clone;
pub(crate) trait Command {
    const NAME: &'static str;
    fn run(&self) -> Result<(), Box<Error>>;
}