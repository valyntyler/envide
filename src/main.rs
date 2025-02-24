use std::fs;

use envide::env::entry::Entry;

fn main() {
    if let Ok(contents) = fs::read_to_string(".env") {
        let entries: Vec<Entry> = contents
            .lines()
            .filter_map(|line| line.try_into().ok())
            .collect();

        for entry in entries {
            println!("{}", entry);
        }
    }
}
