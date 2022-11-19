use std::env;
use std::process;

use fsearch::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or(Config {query: String::from("a"), file_path: String::from("b")});

    if let Err(e) = fsearch::run(config) {
        println!("Error happened: {e}");
        process::exit(1);
    }
}



