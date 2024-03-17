use crate::EntryType::*;
use std::error::Error;

use clap::{App, Arg};
use regex::Regex;
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("findr")
        .version("0.1.0")
        .about("Rust find")
        .arg(
            Arg::with_name("names")
                .short("n")
                .long("name")
                .help("Name")
                .value_name("NAME")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("types")
                .short("t")
                .long("type")
                .value_name("TYPE")
                .takes_value(true)
                .possible_values(&["f", "d", "l"])
                .multiple(true)
                .help("Entry type"),
        )
        .arg(
            Arg::with_name("paths")
                .value_name("PATH")
                .default_value(".")
                .help("Search paths")
                .multiple(true),
        )
        .get_matches();

    let paths = matches.values_of_lossy("paths").unwrap();

    let names = matches
        .values_of_lossy("names")
        .map(|vals| {
            vals.into_iter()
                .map(|name| Regex::new(&name).map_err(|_| format!("Invalid --name \"{}\"", name)))
                .collect::<Result<Vec<Regex>, String>>()
        })
        .transpose()?
        .unwrap_or_default();

    let entry_types = matches
        .values_of_lossy("types")
        .map(|vals| {
            vals.iter()
                .map(|val| match val.as_str() {
                    "d" => Dir,
                    "f" => File,
                    "l" => Link,
                    _ => unreachable!("Invalid type"),
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    Ok(Config {
        paths,
        names,
        entry_types,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("[lib.rs:57] config: {:?}", config);
    for path in config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => println!("{}", entry.path().display()),
            }
        }
    }
    Ok(())
}
