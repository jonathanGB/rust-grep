use std::process;

use minigrep::Config;
use structopt::StructOpt;

fn main() {
    let config = Config::new(Config::from_args);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Problem running the application\n{}", e);
        process::exit(1);
    }
}
