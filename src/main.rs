use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let cl_args: Vec<String> = env::args().collect();
    let config = Config::new(&cl_args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments:\n{}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Problem running the application: {}", e);
        process::exit(1);
    }
}
