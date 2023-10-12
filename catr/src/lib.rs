use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{Arg, ArgAction, Command};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut last_num = 0;
                for (line_num, line) in file.lines().enumerate() {
                    let line = line?;

                    if config.number_lines {
                        println!("{:>6}\t{}", line_num + 1, line)
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:>6}\t{}", last_num, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line)
                    }
                }
            }
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

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Talentaa <talentaa@qq.com>")
        .about("Rust cat")
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(1..)
                .action(ArgAction::Set)
                .default_value("-"),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .action(ArgAction::SetTrue)
                .help("Number lines")
                .conflicts_with("number-nonblank"),
        )
        .arg(
            Arg::new("number-nonblank")
                .short('b')
                .long("number-nonblank")
                .action(ArgAction::SetTrue)
                .help("Number nonblank lines"),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many::<String>("file")
            .unwrap()
            .map(|v| v.to_string())
            .collect::<Vec<_>>(),
        number_lines: matches.get_flag("number"),
        number_nonblank_lines: matches.get_flag("number-nonblank"),
    })
}
