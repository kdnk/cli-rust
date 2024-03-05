use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniqr")
        .version("0.1.0")
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .takes_value(false)
                .help("Show counts"),
        )
        .arg(
            Arg::with_name("in_file")
                .value_name("IN_FILE")
                .default_value("-")
                .help("Input file")
                .required(true),
        )
        .arg(
            Arg::with_name("out_file")
                .value_name("OUT_FILE")
                .help("Output file")
                .required(false),
        )
        .get_matches();

    let in_file = matches.value_of("in_file").unwrap().to_string();
    let out_file = matches.value_of("out_file").map(|s| s.to_string());
    let count = matches.is_present("count");

    Ok(Config {
        in_file,
        out_file,
        count,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file).map_err(|e| format!("{}: {}", config.in_file, e))?;
    let mut line = String::new();

    let mut previous = String::new();
    let mut prev_count = 0;
    loop {
        let bytes = file.read_line(&mut line)?;

        if previous == line || previous.is_empty() {
            previous = line.clone();
            prev_count += 1;
        } else {
            if config.count {
                print!("{} ", prev_count);
            }
            print!("{}", previous);
            previous = line.clone();
            prev_count = 1;
        }

        if bytes == 0 {
            break;
        }

        line.clear();
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
