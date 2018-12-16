extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

enum GuardState {
    Awake,
    Asleep,
}

fn main() -> Result<()> {
    let file = File::open("input.txt")?;

    let mut records = Vec::new();

    for line in BufReader::new(&file).lines() {
        records.push(line.unwrap());
    }

    records.sort();

    let regex_guard_id = Regex::new(r"#(?P<id>\d+)").unwrap();
    let regex_minute_falls_asleep = Regex::new(r":(?P<minute>\d{2})] falls asleep").unwrap();
    let regex_minute_wakes_up = Regex::new(r":(?P<minute>\d{2})] wakes up").unwrap();

    let mut guard_id: u32 = 0;
    let mut guard_state = GuardState::Awake;
    let mut guard_records: HashMap<u32, [u32; 60]> = HashMap::new();

    let mut minute_asleep = 0;

    for line in records.iter() {
        match regex_guard_id.captures(&line) {
            Some(capture) => {
                guard_id = capture["id"].parse::<u32>().unwrap();
                minute_asleep = 0;
            }
            None => match guard_state {
                GuardState::Awake => match regex_minute_falls_asleep.captures(&line) {
                    Some(capture) => {
                        guard_state = GuardState::Asleep;
                        minute_asleep = capture["minute"].parse::<u32>().unwrap();
                    }
                    None => {}
                },
                GuardState::Asleep => match regex_minute_wakes_up.captures(&line) {
                    Some(capture) => {
                        guard_state = GuardState::Awake;
                        let minute_awake = capture["minute"].parse::<u32>().unwrap();

                        for i in minute_asleep..minute_awake {
                            let index = i as usize;
                            guard_records.entry(guard_id).or_insert([0; 60])[index] += 1;
                        }
                    }
                    None => {}
                },
            },
        }
    }

    let mut sleepiest_index = 0;
    let mut sleepiest_minute = 0;
    let mut current_guard_id = 0;
    for (guard_id, record) in guard_records {
        for (index, minute) in record.iter().enumerate() {
            if minute > &sleepiest_minute {
                sleepiest_index = index;
                sleepiest_minute = *minute;
                current_guard_id = guard_id;
            }
        }
    }

    println!("{}", current_guard_id * sleepiest_index as u32);

    Ok(())
}
