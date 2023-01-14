use std::{process, env};
use log::info;
use minigrep::Config;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    info!("Searching for {}", config.query);
    info!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}

