extern crate regex;

use std::env;
use std::error::Error;
use std::fs;

use regex::Regex;

#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
    regex_search: bool,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments: should have a query and a filename parameters (2)");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        let regex_search = args[3..].contains(&String::from("--regex"));

        if regex_search && Regex::new(&query).is_err() {
            return Err("Query is an invalid regular expression");
        }

        Ok(Config {
            query,
            filename,
            case_sensitive,
            regex_search,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(&config.filename)?;
    let search_fn = if config.regex_search {
        search_regex
    } else if config.case_sensitive {
        search
    } else {
        search_case_insensitive
    };

    for (i, line) in search_fn(&config.query, &file_content) {
        println!("Line {}: {}", i, line);
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<(usize, &'a str)> {
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .map(|(i, line)| (i + 1, line))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<(usize, &'a str)> {
    let query = query.to_lowercase();

    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.to_lowercase().contains(&query))
        .map(|(i, line)| (i + 1, line))
        .collect()
}

fn search_regex<'a>(query: &str, content: &'a str) -> Vec<(usize, &'a str)> {
    let regex = Regex::new(query).unwrap();

    content
        .lines()
        .enumerate()
        .filter(|(_, line)| regex.is_match(line))
        .map(|(i, line)| (i + 1, line))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_config() {
        let program = String::from("<program>");
        let query = String::from("<query>");
        let filename = String::from("<filename>");

        let args = vec![program.clone(), query.clone(), filename.clone()];
        let config = Config::new(&args).unwrap();
        assert_eq!(query, config.query);
        assert_eq!(filename, config.filename);
    }

    #[test]
    #[should_panic(
        expected = "Not enough arguments: should have a query and a filename parameters (2)"
    )]
    fn new_bad_config() {
        let program = String::from("<program>");
        let query = String::from("<query>");

        let args = vec![program, query];
        Config::new(&args).unwrap();
    }

    #[test]
    #[should_panic(expected = "Query is an invalid regular expression")]
    fn new_config_invalid_regex() {
        let program = String::from("<program>");
        let query = String::from(r"[a-z\]");
        let filename = String::from("<filename>");
        let regex_flag = String::from("--regex");

        let args = vec![program, query, filename, regex_flag];
        Config::new(&args).unwrap();
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec![(2, "safe, fast, productive.")], search(query, content));
    }

    #[test]
    fn no_result() {
        let query = "404";
        let content = "Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(Vec::<(usize, &str)>::new(), search(query, content));
    }

    #[test]
    fn two_results() {
        let query = "st";
        let content = "Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec![(1, "Rust:"), (2, "safe, fast, productive.")],
            search(query, content)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "RuST";
        let content = "Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec![(1, "Rust:"), (4, "Trust me.")],
            search_case_insensitive(query, content)
        );
    }

    #[test]
    fn case_regex() {
        let query = r"\b[a-zA-Z]{4}\b";
        let content = "Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec![
                (1, "Rust:"),
                (2, "safe, fast, productive."),
                (3, "Pick three.")
            ],
            search_regex(query, content)
        );
    }
}
