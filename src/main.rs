use std::{fs::{self, OpenOptions}, io::{self, Write}};

use envide::env::entry::Entry;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = ".env")]
    path: String,
    
    #[clap(short, long, value_names = [ "KEY", "NEW_VAL" ])]
    replace: Option<Vec<String>>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if let Some(replace) = args.replace {
        let key = &replace[0];
        let new = &replace[1];

        let content: String = fs::read_to_string(&args.path)?
            .lines()
            .map(|line| {
                if let Ok(mut entry) = Entry::try_from(line) {
                    if entry.key == key { entry.val = new; }
                    format!("{}\n", entry)
                } else {
                    line.to_string()
                }
            })
            .collect();

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&args.path)?;

        file.write_all(content.as_bytes())?;
    }

    Ok(())
}
