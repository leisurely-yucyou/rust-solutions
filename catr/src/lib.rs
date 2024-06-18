use std::error::Error;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    // println!("{:?}", config);
    if config.number_lines {
        for filename in config.files {
            match open(&filename) {
                Err(err) => eprintln!("Failed to open {}: {}", filename, err),
                Ok(file) => {
                    // println!("Opened {}", filename);
                    for (index, line) in file.lines().enumerate() {
                        let line = line?;
                        println!("{:>6}\t{}", index+1, line);
                    }
                }
            }
        }
    }
    else if config.number_nonblank_lines {
        for filename in config.files {
            match open(&filename) {
                Err(err) => eprintln!("Failed to open {}: {}", filename, err),
                Ok(file) => {
                    // println!("Opened {}", filename);
                    let mut num: i32 = 1;
                    for line in file.lines() {
                        let line = line?;
                        if line == "" {
                            println!("{}", line);
                        } else {
                            println!("{:>6}\t{}", num, line);
                            num += 1;
                        }
                    }
                }
            }
        }
    }
    else {
        for filename in config.files {
            match open(&filename) {
                Err(err) => eprintln!("Failed to open {}: {}", filename, err),
                Ok(file) => {
                    // println!("Opened {}", filename);
                    for line in file.lines() {
                        let line = line?;
                        println!("{}", line);
                    }
                },
            }
        }
    }
    // dbg!(config);
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                // .value_name("FILES")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                // .help("行番号を付与します")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                // .help("空白行以外に行番号を付与します。")
                .help("Number non-blank lines")
                .takes_value(false),
        )
        .get_matches();
    // if (matches.is_present("number_lines") && matches.is_present("number_nonblank_lines")) {
    //     Error(Config{"The argument '--number-nonblank' cannot be used with '--number'"})
    // }
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}