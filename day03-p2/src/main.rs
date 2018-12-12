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
    ids: Vec<i32>,
    map: HashMap<Coordinates, Vec<i32>>,
}

impl Claims {
    fn add(&mut self, coordinates: Coordinates, id: i32, w: i32, h: i32) {
        self.ids.push(id);

        for i in 0..w {
            for j in 0..h {
                let c = Coordinates {
                    x: coordinates.x + i,
                    y: coordinates.y + j,
                };
                self.map.entry(c).or_default().push(id);
            }
        }
    }
}

fn main() -> Result<()> {
    let file = File::open("input.txt")?;

    let re = Regex::new(r"^#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<h>\d+)x(?P<w>\d+)$").unwrap();

    let mut claims = Claims {
        ids: Vec::new(),
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
            caps["id"].parse::<i32>().unwrap(),
            caps["h"].parse::<i32>().unwrap(),
            caps["w"].parse::<i32>().unwrap(),
        );
    }

    let mut good_claims = Vec::new();
    for id in claims.ids.iter() {
        good_claims.push(id);
    }

    for (_, ids) in claims.map.iter_mut() {
        if ids.len() <= 1 {
            continue;
        }

        for id in ids {
            good_claims.retain(|&x| x != id);
        }
    }

    println!("{}", good_claims[0]);

    Ok(())
}
