use anyhow::Context;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn run_stats(input: &Path) -> anyhow::Result<()> {
    let file = File::open(input).with_context(|| format!("open file at `{:?}`", input))?;

    let reader = BufReader::new(file);
    let mut num_of_words = 0;
    let mut num_of_lines = 0;
    let mut num_of_chars = 0;

    for (index, line_result) in reader.lines().enumerate() {
        let line =
            line_result.with_context(|| format!("read line {} from `{:?}`", index + 1, input))?;

        for word in line.trim().split_whitespace() {
            num_of_words += 1;
            num_of_chars += word.chars().count();
        }
        num_of_lines += 1;
    }

    println!(
        "File: `{:?}`\nLines: {}\nWords: {}\nCharacters: {}",
        input, num_of_lines, num_of_words, num_of_chars
    );

    Ok(())
}
