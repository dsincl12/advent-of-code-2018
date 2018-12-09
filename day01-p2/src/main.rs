use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let file = File::open("input.txt")?;

    let mut numbers = Vec::new();

    for line in BufReader::new(&file).lines() {
        let number: i64 = line?.parse().expect("unable to parse number");

        numbers.push(number);
    }

    let mut sum = 0;
    let mut seen = HashMap::new();

    seen.insert(sum, true);

    loop {
        for number in numbers.iter() {
            sum += number;

            if seen.contains_key(&sum) {
                println!("{}", sum);
                return Ok(());
            }

            seen.insert(sum, true);
        }
    }
}
