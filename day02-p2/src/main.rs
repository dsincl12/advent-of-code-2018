use std::fs::File;
use std::io::{BufRead, BufReader, Result};

struct DiffResult {
    diff_count: u32,
    position: usize,
}

fn main() -> Result<()> {
    let file = File::open("input.txt")?;

    let mut lines = Vec::new();
    for line in BufReader::new(&file).lines() {
        lines.push(line.unwrap());
    }

    for i in 0..lines.len() {
        for j in 1..lines.len() {
            let result = calc_diff(&lines[i], &lines[j]);

            if result.diff_count == 1 {
                println!(
                    "{}{}",
                    &lines[i][0..result.position],
                    &lines[i][result.position + 1..lines[i].len()]
                );

                return Ok(());
            }
        }
    }

    Ok(())
}

fn calc_diff(line1: &str, line2: &str) -> DiffResult {
    let mut position = 0;
    let mut diff_count = 0;

    for i in 0..line1.len() {
        if line1.chars().nth(i) == line2.chars().nth(i) {
            continue;
        }

        position = i;
        diff_count += 1;
    }

    DiffResult {
        diff_count,
        position,
    }
}
