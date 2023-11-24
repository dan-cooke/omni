use clap::Command;
use omni::generate::Generate;

fn main() {
    let omni = Command::new("omni").subcommand(Generate::new());
    let args = omni.get_matches();
    println!("{:?}", args);
}
