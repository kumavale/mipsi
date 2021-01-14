#[macro_use]
extern crate clap;

mod cli;
mod lexer;
mod parser;
mod repl;
mod token;

use clap::{Arg, App};

fn main() {
    let matches = App::new("mipsi")
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("file").multiple(true))
        .get_matches();

    // REPL
    if matches.values_of("file").is_none() {
        repl::run();
        return;
    }

    // CLI
    let files: Vec<&str> = matches.values_of("file").unwrap().collect();
    cli::run(&files);
}

