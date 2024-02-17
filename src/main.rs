mod cli;
mod lexer;
mod parser;
mod repl;
mod token;

use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(value_name = "FILE")]
    files: Vec<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    // REPL
    if cli.files.is_empty() {
        repl::run();
        return;
    }

    // CLI
    cli::run(&cli.files);
}

