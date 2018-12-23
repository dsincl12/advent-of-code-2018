use std::fs::File;
use std::io::{BufRead, BufReader, Result};

extern crate rayon;

use rayon::prelude::*;

fn main() -> Result<()> {
    let file = File::open("input.txt")?;

    for l in BufReader::new(&file).lines() {
        let line = l.unwrap();

        static UNIT_TYPES: [char; 26] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];

        let mut results: Vec<usize> = UNIT_TYPES
            .par_iter()
            .map(|unit_type| {
                let mut length = 0;

                stacker::grow(8192 * 8192, || {
                    length = merge_polymers(1, remove_unit_type(*unit_type, &line));
                });

                length
            })
            .collect();

        results.sort();

        println!("{}", results[0]);
    }

    Ok(())
}

fn remove_unit_type(ch: char, line: &str) -> String {
    line.chars()
        .filter(|&c| !c.eq_ignore_ascii_case(&ch))
        .collect()
}

fn merge_polymers(mut index: usize, mut line: String) -> usize {
    if index < line.chars().count() {
        let ch1 = line.chars().nth(index).unwrap();
        let ch2 = line.chars().nth(index - 1).unwrap();

        if merge_possible(ch1, ch2) {
            line.remove(index);
            line.remove(index - 1);

            if index > 1 {
                index -= 1;
            }

            return merge_polymers(index, line);
        }

        return merge_polymers(index + 1, line);
    }

    line.len()
}

fn merge_possible(ch1: char, ch2: char) -> bool {
    if ch1.is_lowercase() && ch2 == ch1.to_uppercase().next().unwrap() {
        return true;
    }

    if ch1.is_uppercase() && ch2 == ch1.to_lowercase().next().unwrap() {
        return true;
    }

    false
}
