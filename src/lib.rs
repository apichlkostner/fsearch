use std::error::Error;
use std::io;
use std::fs;
use regex::Regex;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open(&config.file_path)?;
    let reader = io::BufReader::new(file);

    let lines = search(&config.query, reader, config.use_regex)?;
    for line in lines {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub use_regex: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let mut use_regex  = false;
        let query = args[1].clone();
        let file_path = args[2].clone();
        if args.len() > 3 {
            use_regex = args[3] == "-r";
        }
    
        Ok(Config{query, file_path, use_regex})
    }
}

pub fn search(query: &str, bufreader: impl io::BufRead, use_regex: bool) -> Result<Vec<String>, Box<dyn Error>> {
    let mut results = Vec::new();
    let lines = bufreader.lines();

    if use_regex {
        let re = Regex::new(query)?;
        for line in lines {
            if let Ok(l) = line {
                if re.is_match(&l) {
                    results.push(l);
                }
            }
        }
    } else {
        for line in lines {
            if let Ok(l) = line {
                if l.contains(query) {
                    results.push(l);
                }
            }
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.".as_bytes();
        let bufread = io::BufReader::new(contents);

        let res = search(query, bufread, false).unwrap();
        assert_eq!(vec!["safe, fast, productive."], res);
    }

    #[test]
    fn one_result_with_star() {
        let query = "du.*ct";
        let contents = "\
Rust:
safe, fast, productive.
safe, fast, produ.*ctive.
safe, fast, produ.*fctive.
Pick three.".as_bytes();

        let bufread = io::BufReader::new(contents);
        let res = search(query, bufread, false).unwrap();
        assert_eq!(vec!["safe, fast, produ.*ctive."], res);
    }

    #[test]
    fn one_result_regex() {
        let query = "fa.*ct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.".as_bytes();

        let bufread = io::BufReader::new(contents);
        let res = search(query, bufread, true).unwrap();
        assert_eq!(vec!["safe, fast, productive."], res);
    }

    #[test]
    fn error_regex() {
        let query = "**"; // invalid regex
        let contents = "\
Rust:
safe, fast, productive.
Pick three.".as_bytes();

        let bufread = io::BufReader::new(contents);
        let res = search(query, bufread, true);
        assert!(res.is_err());
    }
}