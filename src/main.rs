use std::fs;

use envide::env::entry::Entry;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = ".env")]
    path: String,
}

pub fn get_value<'a>(key: &str, entries: &'a Vec<Entry>) -> Option<&'a str> {
    for entry in entries {
        if entry.key == key {
            return Some(&entry.val)
        }
    }

    None
}

fn main() {
    let args = Args::parse();

    if let Ok(contents) = fs::read_to_string(args.path) {
        let entries: Vec<Entry> = contents
            .lines()
            .filter_map(|line| line.try_into().ok())
            .collect();

        for entry in &entries {
            println!("{}", entry);
        }

        if let Some(val) = get_value("JEFF", &entries) {
            println!("{}", val);
        }
    }
}
