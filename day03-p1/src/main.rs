extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader, Result};

#[derive(Clone, Debug)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Eq for Coordinates {}

impl PartialEq for Coordinates {
    fn eq(&self, other: &Coordinates) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Coordinates {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write_i32(self.x + self.y);
        state.finish();
    }
}

#[derive(Clone, Debug)]
struct Claims {
    map: HashMap<Coordinates, i32>,
}

impl Claims {
    fn add(&mut self, coordinates: Coordinates, w: i32, h: i32) {
        for i in 0..w {
            for j in 0..h {
                *self
                    .map
                    .entry(Coordinates {
                        x: coordinates.x + i,
                        y: coordinates.y + j,
                    })
                    .or_insert(0) += 1;
            }
        }
    }
}

fn main() -> Result<()> {
    let file = File::open("input.txt")?;

    let re = Regex::new(r"^#\d+ @ (?P<x>\d+),(?P<y>\d+): (?P<h>\d+)x(?P<w>\d+)$").unwrap();

    let mut claims = Claims {
        map: HashMap::new(),
    };

    for l in BufReader::new(&file).lines() {
        let line = l.unwrap();
        let caps = re.captures(&line).unwrap();

        let coordinates = Coordinates {
            x: caps["x"].parse::<i32>().unwrap(),
            y: caps["y"].parse::<i32>().unwrap(),
        };

        claims.add(
            coordinates,
            caps["h"].parse::<i32>().unwrap(),
            caps["w"].parse::<i32>().unwrap(),
        );
    }

    let mut count = 0;
    for (_key, value) in claims.map.drain() {
        if value > 1 {
            count += 1;
        }
    }

    println!("{}", count);

    Ok(())
}
