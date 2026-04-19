use anyhow::Context;
use std::fs::File;
use std::{
    io::{BufRead, BufReader},
    path::Path,
};

pub fn run_search(input: &Path, pattern: &str, case_insensitive: bool) -> anyhow::Result<()> {
    let file = File::open(input).with_context(|| format!("open file at `{:?}`", input))?;

    let reader = BufReader::new(file);
    let mut does_not_contain_pattern = true;

    for (index, line_result) in reader.lines().enumerate() {
        let line =
            line_result.with_context(|| format!("read line {} from `{:?}`", index + 1, input))?;

        let matches = if case_insensitive {
            line.to_lowercase().contains(&pattern.to_lowercase())
        } else {
            line.contains(pattern)
        };
        if matches {
            println!("{}: {}", index + 1, line);
            does_not_contain_pattern = false;
        }
    }

    if does_not_contain_pattern {
        println!("No matches found.");
    }
    Ok(())
}
