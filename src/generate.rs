use clap::Command;
use omni_parser::parse_file;

pub struct Generate {}

impl Generate {
    pub fn run(&self) -> Result<(), String> {
        // Hardcode main.omni for now
        let file = parse_file("main.omni");

        print!("{:?}", file);
        Ok(())
    }
    pub fn command() -> Command {
        Command::new("generate").visible_alias("g")
    }
}
