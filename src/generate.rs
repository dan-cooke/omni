use clap::Command;

pub struct Generate {}

impl Generate {
    pub fn new() -> Command {
        Command::new("generate").visible_alias("g")
    }
}
