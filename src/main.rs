use std::{env,process};
use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err|{
        eprintln!("Problems parsing input: {err}");
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config){
        eprintln!("there was a problem reading the file: {err}");
        process::exit(1);
    }
}

