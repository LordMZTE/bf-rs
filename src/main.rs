use std::fs::File;

use anyhow::{Context, Result};
use clap::{App, Arg};

mod interpreter;
mod tokenizer;

const ARG_FAIL: &str = "failed to get arg";

fn main() -> Result<()> {
    let matches = App::new("bf-rs")
        .author("LordMZTE")
        .about("Brainfuck interpreter written in rust")
        .arg(
            Arg::with_name("file")
                .index(1)
                .help("the file to interpret"),
        )
        .arg(
            Arg::with_name("tree")
                .help("emit a json syntax tree instead of running the program")
                .short("t"),
        )
        .arg(
            Arg::with_name("ast")
                .help("run a json tree instead of brainfuck code")
                .short("a"),
        )
        .get_matches();

    let mut file = File::open(matches.value_of("file").context(ARG_FAIL)?)?;

    let tree = if matches.is_present("ast") {
        serde_json::from_reader(file)?
    } else {
        let tokens = tokenizer::tokenize(&mut file).context("tokenization failed")?;
        tokenizer::Tree::parse(tokens.iter())
    };

    if matches.is_present("tree") {
        serde_json::to_writer_pretty(std::io::stdout(), &tree)?;
    } else {
        interpreter::run_new(tree.iter(), &mut std::io::stdout(), &mut std::io::stdin());
    }

    Ok(())
}
