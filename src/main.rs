use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
};

use envide::env::entry::Entry;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = ".env")]
    path: String,

    #[arg(short, long, default_value_t = false)]
    new: bool,

    #[arg(short = 'x', long, default_value_t = false)]
    exclusive: bool,

    #[arg(short, long, default_value_t = false)]
    replace: bool,

    #[arg(num_args = 2, value_names = [ "KEY", "VALUE" ])]
    trailing: Vec<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let key = &args.trailing[0];
    let val = &args.trailing[1];

    let mut content: String = fs::read_to_string(&args.path)?;

    if args.replace {
        content = content
            .lines()
            .filter(|line| match Entry::try_from(*line) {
                Err(_) => true,
                Ok(entry) => match entry.key == key {
                    true => false,
                    false => true,
                },
            })
            .map(|line| format!("{}\n", line))
            .collect()
    }

    if args.new {
        let mut ok = true;
        if args.exclusive {
            content.lines().for_each(|line| {
                if let Ok(entry) = Entry::try_from(line) {
                    if entry.key == key {
                        ok = false;
                    }
                }
            });
        }

        if ok {
            let entry = Entry { key, val };
            content = format!("{}{}\n", content, entry);
        }
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&args.path)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}
