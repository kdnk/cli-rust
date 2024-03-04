use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Kodai Nakamura")
        .about("wc")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .multiple(true),
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .value_name("WORDS")
                .takes_value(false)
                .help("Show word count"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .takes_value(false)
                .help("Show byte count"),
        )
        .arg(
            Arg::with_name("chars")
                .short("m")
                .long("chars")
                .value_name("CHARS")
                .help("Show character count")
                .takes_value(false)
                .conflicts_with("bytes"),
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .value_name("LINES")
                .takes_value(false)
                .help("Show line count"),
        )
        .get_matches();

    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let mut chars = matches.is_present("chars");

    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
        chars = false;
    }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_chars = 0;
    let mut total_bytes = 0;

    for filename in config.files.clone() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", &filename, err),
            Ok(file) => {
                let info = count(file)?;

                total_lines += info.num_lines;
                total_words += info.num_words;
                total_chars += info.num_chars;
                total_bytes += info.num_bytes;

                render(&info, &config, &filename);
            }
        }
    }

    if config.files.len() > 1 {
        let total_info = FileInfo {
            num_lines: total_lines,
            num_words: total_words,
            num_chars: total_chars,
            num_bytes: total_bytes,
        };
        render(&total_info, &config, "total");
    }

    Ok(())
}

fn render(info: &FileInfo, config: &Config, filename: &str) {
    if config.lines {
        print!("{:>8}", info.num_lines)
    }

    if config.words {
        print!("{:>8}", info.num_words)
    }

    if config.chars {
        print!("{:>8}", info.num_chars);
    } else if config.bytes {
        print!("{:>8}", info.num_bytes);
    }

    if filename != "-" {
        println!(" {}", &filename);
    } else {
        println!();
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_chars = 0;
    let mut num_bytes = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }

        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected)
    }
}
