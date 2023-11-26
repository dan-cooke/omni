use clap::Command;
use omni_codegen::CodeGenerator;
use omni_typescript::ssdk::TypescriptSSDKGenerator;

fn main() {
    let mut omni = Command::new("omni")
        .subcommand(Command::new("generate"))
        .visible_alias("g");

    let args = omni.clone().get_matches();

    let (command, _args) = if let Some((command, args)) = args.subcommand() {
        (command, args)
    } else {
        omni.print_help().unwrap();
        return;
    };

    match command {
        "generate" => CodeGenerator::new(TypescriptSSDKGenerator::new())
            .run()
            .unwrap(),
        &_ => {
            print!("Do nothing");
        }
    };
}
