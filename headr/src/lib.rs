use std::error::Error;

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Kodai Nakamura")
        .about("Rust head")
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Number of lines [default: 10]")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .takes_value(true)
                .conflicts_with("lines")
                .help("Number of bytes"),
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .multiple(true)
                .help("Input file(s)")
                .default_value("-"),
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();

    let str_lines = matches.value_of("lines").unwrap();
    if let Err(_e) = parse_positive_int(str_lines) {
        eprintln!("illegal line count -- {}", str_lines);
    }
    let lines = parse_positive_int(str_lines)?;

    let str_bytes = matches.value_of("bytes");
    let bytes = match str_bytes {
        Some(x) => {
            if let Err(_e) = parse_positive_int(x) {
                eprintln!("illegal byte count -- {}", str_bytes.unwrap());
            }
            Some(parse_positive_int(x).unwrap())
        }
        None => None,
    };

    Ok(Config {
        files,
        lines,
        bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    return match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    };
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
