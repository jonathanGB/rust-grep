# Rust - minigrep

*Simple implementation of grep written in Rust. Based on the tutorial chapter in the excellent [Rust Programming Book](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).*

## How to run?
```
$ git clone https://github.com/jonathanGB/rust-grep.git
$ cd rust-grep
$ [CASE_INSENSITIVE=] cargo run <query> <filename> [--regex]
```

where `query` is the keyword you are grepping inside a file which path is `filename`. An optional parameter `--regex` tells the program that `query` should be interpreted as a regular expression, not simply as a substring. In case the regex flag is not used, it is still possible to denote a case insensitive search by setting the environment variable `CASE_INSENSITIVE` (the value doesn't matter). Minigrep is also currently limited in that we can only grep one file.

### Examples

```
$ cargo run are poem.txt
Line 1: I'm nobody! Who are you?

$ CASE_INSENSITIVE= cargo run are poem.txt
Line 1: I'm nobody! Who are you?
Line 2: Are you nobody, too?

$ cargo run '\b[a-z]{3}re' poem.txt --regex
Line 3: Then there's a pair of us - don't tell!
```