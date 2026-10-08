use colored::Colorize;

use clap::Parser;
use dotenvy::from_filename;

use crate::core::Printable;
mod core;

#[derive(Debug, Clone, Parser)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    size: Option<usize>,
}

fn load_env() {
    let env_loaded = from_filename(".env");
    if env_loaded.is_err() {
        panic!("env not loaded")
    }
    let test_value = std::env::var("TEST_ENV_VAR");
    if test_value.is_err() {
        panic!("env not loaded !");
    }
    println!("{}", "env loaded".blue());
}

fn main() {
    load_env();
    let args = Args::parse();

    println!("{}", " -- Welcome in Rubik solver --".yellow());

    let size = args.size.unwrap_or(3);
    let mut cube = core::Cube::new(size);

    // cube.print();
    cube.scramble();
}
