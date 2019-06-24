extern crate regex;
extern crate structopt;

use std::error::Error;
use std::fs;

use regex::Regex;
use regex::RegexBuilder;
use structopt::StructOpt;

fn parse_regex(src: &str) -> RegexBuilder {
    RegexBuilder::new(src)
}

#[derive(StructOpt)]
#[structopt(name = "minigrep")]
pub struct Config {
    /// Query to search for.
    #[structopt(parse(from_str = "parse_regex"))]
    query: RegexBuilder,

    /// In what file to search for.
    filename: String,

    /// Specify whether or not the query is case sensitive.
    /// It is case sensitive by default.
    #[structopt(long)]
    case_insensitive: bool,
}

impl Config {
    pub fn new<F>(get_args: F) -> Self
    where
        F: FnOnce() -> Config,
    {
        let mut config = get_args();
        config.query.case_insensitive(config.case_insensitive);
        config
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(&config.filename)?;
    let regex_query = config.query.build()?;

    for (i, line) in search(&regex_query, &file_content) {
        println!("Line {}: {}", i, line);
    }

    Ok(())
}

fn search<'a>(query: &Regex, content: &'a str) -> Vec<(usize, &'a str)> {
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| query.is_match(line))
        .map(|(i, line)| (i + 1, line))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_config() {
        let query = RegexBuilder::new(r"[a-z]");
        let filename = String::from("poem.txt");
        let case_insensitive = false;

        let config = Config::new(|| Config{query, filename, case_insensitive});
        assert!(run(config).is_ok());     
    }

    #[test]
    fn new_config_invalid_regex() {
        let query = RegexBuilder::new(r"[a-z\]");
        let filename = String::from("poem.txt");
        let case_insensitive = false;

        let config = Config::new(|| Config{query, filename, case_insensitive});
        assert!(run(config).is_err());
    }

    #[test]
    fn one_result() {
        let query = Regex::new("duct").unwrap();
        let content = "Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec![(2, "safe, fast, productive.")], search(&query, content));
    }

    #[test]
    fn no_result() {
        let query = Regex::new("404").unwrap();
        let content = "Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(Vec::<(usize, &str)>::new(), search(&query, content));
    }

    #[test]
    fn two_results() {
        let query = Regex::new("st").unwrap();
        let content = "Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec![(1, "Rust:"), (2, "safe, fast, productive.")],
            search(&query, content)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = RegexBuilder::new("RuST").case_insensitive(true).build().unwrap();
        let content = "Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec![(1, "Rust:"), (4, "Trust me.")],
            search(&query, content)
        );
    }

    #[test]
    fn case_regex() {
        let query = Regex::new(r"\b[a-zA-Z]{4}\b").unwrap();
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
            search(&query, content)
        );
    }
}
