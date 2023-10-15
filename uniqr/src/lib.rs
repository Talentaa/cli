use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

use clap::{Arg, ArgAction, Command};

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("uniqr")
        .version("0.1.0")
        .author("Talentaa <talentaa@qq.com>")
        .about("Rust uniq")
        .arg(
            Arg::new("in_file")
                .value_name("IN_FILE")
                .default_value("-")
                .help("Input file"),
        )
        .arg(
            Arg::new("out_file")
                .value_name("OUT_FILE")
                .help("Output file"),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .action(ArgAction::SetTrue)
                .help("Show counts"),
        )
        .get_matches();

    Ok(Config {
        in_file: matches.get_one("in_file").cloned().unwrap(),
        out_file: matches.get_one("out_file").cloned(),
        count: matches.get_flag("count"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    match open(&config.in_file) {
        Err(e) => eprintln!("{}: {}", config.in_file, e),
        Ok(mut file) => {
            let mut line = String::new();
            let mut previous = String::new();
            let mut count: u64 = 0;

            let mut out_file: Box<dyn Write> = match &config.out_file {
                Some(out_name) => Box::new(File::create(out_name)?),
                _ => Box::new(io::stdout()),
            };

            let mut print = |count: u64, text: &str| -> MyResult<()> {
                if count > 0 {
                    if config.count {
                        write!(out_file, "{:>4} {}", count, text)?;
                    } else {
                        write!(out_file, "{}", text)?;
                    }
                }
                Ok(())
            };

            while let Ok(bytes) = file.read_line(&mut line) {
                if bytes == 0 {
                    break;
                }
                if line.trim_end() != previous.trim_end() {
                    print(count, &previous)?;
                    (previous, line) = (line, previous);
                    count = 0;
                }

                count += 1;
                line.clear();
            }
            print(count, &previous)?;
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
