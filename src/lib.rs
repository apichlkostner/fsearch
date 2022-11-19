use std::error::Error;
use std::fs;
use regex::Regex;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(&config.file_path)?;

    let lines = search(&config.query, &file, config.use_regex)?;
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

pub fn search<'a>(query: &str, contents: &'a str, use_regex: bool) -> Result<Vec<&'a str>, Box<dyn Error>> {
    let mut results = Vec::new();

    if use_regex {
        let re = Regex::new(query)?;
        for line in contents.lines() {
            if re.is_match(line) {
                results.push(line);
            }
        }
    } else {
        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
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
Pick three.";

        let res = search(query, contents, false).unwrap();
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
Pick three.";

        let res = search(query, contents, false).unwrap();
        assert_eq!(vec!["safe, fast, produ.*ctive."], res);
    }

    #[test]
    fn one_result_regex() {
        let query = "fa.*ct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let res = search(query, contents, true).unwrap();
        assert_eq!(vec!["safe, fast, productive."], res);
    }

    #[test]
    fn error_regex() {
        let query = "**"; // invalid regex
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let res = search(query, contents, true);
        assert!(res.is_err());
    }
}