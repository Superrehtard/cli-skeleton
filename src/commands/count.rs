use anyhow::Context;
use std::fs::File;
use std::{
    io::{BufRead, BufReader},
    path::Path,
};

pub fn run_count(input: &Path, words: bool) -> anyhow::Result<()> {
    let file_at_path = File::open(input).with_context(|| format!("open file at `{:?}`", input))?;

    let reader = BufReader::new(file_at_path);
    let mut num_of_words = 0;
    let mut num_of_lines = 0;

    for (index, line_result) in reader.lines().enumerate() {
        let line =
            line_result.with_context(|| format!("read line {} from `{:?}`", index + 1, input))?;

        num_of_words += line.split_whitespace().count();
        num_of_lines += 1;
    }

    if words {
        println!("{} words in `{:?}`", num_of_words, input);
    } else {
        println!("`{}` lines in `{:?}`", num_of_lines, input);
    }

    Ok(())
}
