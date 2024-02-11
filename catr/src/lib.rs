use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Kodai Nakamura")
        .about("Rust cat")
        .arg(Arg::with_name("file").multiple(true).default_value("-"))
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .takes_value(false)
                .conflicts_with("number"),
        )
        .get_matches();

    let files = matches.values_of_lossy("file").unwrap();
    let number_lines = matches.is_present("number");
    let number_nonblank_lines = matches.is_present("number_nonblank");

    Ok(Config {
        files,
        number_lines,
        number_nonblank_lines,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(content) => {
                let mut last_num = 0;
                for (line_num, line_result) in content.lines().enumerate() {
                    let line = line_result?;

                    if config.number_lines {
                        println!("{:6}\t{}", line_num + 1, line);
                    } else if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                        } else {
                            last_num += 1;
                            println!("{:6}\t{}", last_num, line);
                        }
                    } else {
                        println!("{}", line);
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
