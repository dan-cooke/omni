use clap::Command;
use omni::generate::Generate;

fn main() {
    let mut omni = Command::new("omni").subcommand(Generate::command());

    let args = omni.clone().get_matches();

    let (command, _args) = if let Some((command, args)) = args.subcommand() {
        (command, args)
    } else {
        omni.print_help().unwrap();
        return;
    };

    match command {
        "generate" => Generate::run(),
        &_ => {
            print!("Do nothing");
        }
    }
}
