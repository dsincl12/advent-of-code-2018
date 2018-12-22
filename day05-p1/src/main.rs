use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let file = File::open("input.txt")?;

    for l in BufReader::new(&file).lines() {
        let line = l.unwrap();

        stacker::grow(8192 * 8192, || {
            merge_polymers(1, line);
        })
    }

    Ok(())
}

fn merge_polymers(mut index: usize, mut line: String) {
    while index < line.chars().count() {
        let ch1 = line.chars().nth(index).unwrap();
        let ch2 = line.chars().nth(index - 1).unwrap();

        if merge_possible(&ch1, &ch2) {
            line.remove(index);
            line.remove(index - 1);

            if index > 1 {
                index -= 1;
            }

            return merge_polymers(index, line);
        }

        return merge_polymers(index + 1, line);
    }

    println!("{}", line.len());
}

fn merge_possible(ch1: &char, ch2: &char) -> bool {
    if ch1.is_lowercase() {
        if *ch2 == ch1.to_uppercase().next().unwrap() {
            return true;
        }
    }

    if ch1.is_uppercase() {
        if *ch2 == ch1.to_lowercase().next().unwrap() {
            return true;
        }
    }

    return false;
}
