extern crate clap;


use std::env;
use clap::{App, Arg, SubCommand};


fn main() {
    let app = App::new("deversion")
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .about(env!("CARGO_PKG_DESCRIPTION"))
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

    let mut init_directory = "";
    if let Some(matches) = matches.subcommand_matches("init") {
        if let Some(matches) = matches.value_of("input") {
            init_directory = matches;
        }
    }

    match matches.subcommand_name() {
        Some("init") => init(init_directory),
        Some("save") => save(),
        None         => println!("No subcommand was used"),
        _            => println!("Some other subcommand was used"),
    }
}


fn init(directory: &str) {
    println!("'dev init' was run.");
    if directory.is_empty() {
        let path = env::current_dir().unwrap();
        println!("{}", path.display());
    } else {
        println!("{}", directory);
    }
}


fn save() {
    println!("'dev save' was run.");
}
