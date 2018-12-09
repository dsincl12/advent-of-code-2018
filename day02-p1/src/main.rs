use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let file = File::open("input.txt")?;

    let mut twos: u32 = 0;
    let mut threes: u32 = 0;

    for line in BufReader::new(&file).lines() {
        let current = line.unwrap();
        let mut char_count = HashMap::new();

        for ch in current.chars() {
            if !char_count.contains_key(&ch) {
                let count = count_char(ch, &current);

                if !char_count.values().any(|&v| v == count) {
                    char_count.insert(ch, count);
                }
            }
        }

        for (_, v) in char_count.drain() {
            if v == 2 {
                twos += 1;
            }

            if v == 3 {
                threes += 1;
            }
        }
    }

    println!("{}", twos * threes);

    Ok(())
}

fn count_char(ch: char, line: &str) -> u32 {
    let mut count = 0;

    for c in line.chars() {
        if c == ch {
            count += 1;
        }
    }

    count
}
