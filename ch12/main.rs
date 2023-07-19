use std::{env, process};

// mod lib;

fn main() {
    let config = ch12::Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = ch12::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
