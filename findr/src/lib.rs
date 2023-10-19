use clap::{builder::PossibleValue, Arg, ArgAction, Command, ValueEnum};
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Dir, Self::File, Self::Link]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Dir => PossibleValue::new("d"),
            Self::File => PossibleValue::new("f"),
            Self::Link => PossibleValue::new("l"),
        })
    }
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("findr")
        .version("0.1.0")
        .author("Talentaa <talentaa@qq.com>")
        .about("Rust find")
        .arg(
            Arg::new("paths")
                .value_name("PATH")
                .num_args(1..)
                .default_value(".")
                .help("Search paths"),
        )
        .arg(
            Arg::new("names")
                .value_name("NAME")
                .short('n')
                .long("name")
                .value_parser(Regex::new)
                .action(ArgAction::Append)
                .num_args(0..)
                .help("Name"),
        )
        .arg(
            Arg::new("entry_types")
                .value_name("TYPE")
                .short('t')
                .long("type")
                .value_parser(clap::value_parser!(EntryType))
                .action(ArgAction::Append)
                .num_args(0..)
                .help("Entry type"),
        )
        .get_matches();

    Ok(Config {
        paths: matches
            .get_many("paths")
            .expect("paths required")
            .cloned()
            .collect(),

        names: matches
            .get_many("names")
            .unwrap_or_default()
            .cloned()
            .collect(),

        entry_types: matches
            .get_many("entry_types")
            .unwrap_or_default()
            .cloned()
            .collect(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let type_filter = |entry: &DirEntry| {
        config.entry_types.is_empty()
            || config
                .entry_types
                .iter()
                .any(|entry_type| match entry_type {
                    EntryType::Link => entry.file_type().is_symlink(),
                    EntryType::Dir => entry.file_type().is_dir(),
                    EntryType::File => entry.file_type().is_file(),
                })
    };

    let name_filter = |entry: &DirEntry| {
        config.names.is_empty()
            || config
                .names
                .iter()
                .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in config.paths {
        let entries: Vec<String> = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect();

        println!("{}", entries.join("\n"));
    }
    Ok(())
}
