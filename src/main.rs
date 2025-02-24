use std::fs;

use envide::env::entry::Entry;

fn main() {
    if let Ok(contents) = fs::read_to_string(".env") {
        let entries: Vec<Entry> = contents
            .lines()
            .filter_map(|line| {
                line.split_once('=').and_then(|(lhs, rhs)| {
                    Some(Entry {
                        key: lhs.to_string(),
                        val: rhs.to_string()
                    })
                })
            })
            .collect();

        for entry in entries {
            println!("{}", entry);
        }
    }
}
