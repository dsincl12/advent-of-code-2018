use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let file = File::open("input.txt")?;

    let mut sum = 0;
    for line in BufReader::new(file).lines() {
        let number: i32 = line?.parse().expect("unable to parse number");
        sum += number;
    }

    println!("{}", sum);

    Ok(())
}
