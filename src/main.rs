use std::env;
use std::process;

use fsearch::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|e| {
        eprintln!("Error in config: {e}");
        process::abort();
    });

    if let Err(e) = fsearch::run(config) {
        eprintln!("Error happened: {e}");
        process::exit(1);
    }
}



