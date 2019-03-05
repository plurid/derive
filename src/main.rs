extern crate clap;

use clap::{App, Arg, SubCommand};


fn main() {
    let app = App::new("deversion")
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .about("Always-active, character-for-character version control system")
                .help_message("Print help")
                .version_message("Print version")
                .subcommand(SubCommand::with_name("init")
                    .about("Initialize repository")
                    .arg(Arg::with_name("input")
                        .required(false)))
                .subcommand(SubCommand::with_name("save")
                    .about("Save specific version in time")
                    .arg(Arg::with_name("description")
                        .short("d")
                        .long("description")
                        .help("description of the save")
                        .required(false)));

    let matches = app.get_matches();

    match matches.subcommand_name() {
        Some("init") => println!("'dev init' was run."),
        Some("save") => println!("'dev save' was run."),
        None         => println!("No subcommand was used"),
        _            => println!("Some other subcommand was used"),
    }
}
