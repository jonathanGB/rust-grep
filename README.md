# Rust - minigrep

*Simple implementation of grep written in Rust. Based on the tutorial chapter in the excellent [Rust Programming Book](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).*

## How to run?
```
$ git clone https://github.com/jonathanGB/rust-grep.git
$ cd rust-grep
$ cargo run <query> <filename> [--case_insensitive]
```

where `query` is the keyword you are grepping inside a file which path is `filename`. It is possible to denote a case insensitive search by using the `--case_insensitive` flag. Minigrep is also currently limited in that we can only grep one file.

### Examples

```
$ cargo run are poem.txt
Line 1: I'm nobody! Who are you?

$ cargo run are poem.txt --case_insensitive
Line 1: I'm nobody! Who are you?
Line 2: Are you nobody, too?

$ cargo run '\b[a-z]{3}re' poem.txt
Line 3: Then there's a pair of us - don't tell!
```