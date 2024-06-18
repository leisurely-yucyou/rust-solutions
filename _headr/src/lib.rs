use clap::{App, Arg, AppSettings};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

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
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust head")
        .setting(AppSettings::DisableHelpFlags)
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("lines")
                .value_name("LINES")
                .short("n")
                .long("lines")
                .help("Number of lines")
                // .takes_value(true)
                // .conflicts_with("bytes")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .short("c")
                .long("bytes")
                .help("Number of bytes")
                .takes_value(true)
                .conflicts_with("lines"),
        )
        .get_matches();

    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        // .map_err(|e| format!("illegal line count -- {}", e))?;
        .map_err(|e| format!("error: invalid value '{}' for \
        '--lines <LINES>': invalid digit found in string", e))?;
        // .unwrap_or(10);

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        // .map_err(|e| format!("illegal byte count -- {}", e))?;
        .map_err(|e| format!("invalid value '{}' for \
        '--bytes <BYTES>': invalid digit found in string", e))?;
        
    
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes,
    })

}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                let mut num = String::new();
                if filename != "-" && filename !=""  && config.files.len() > 1 {
                    println!("==> {} <==", &filename)
                };
                match config.bytes {
                    None => {
                        let mut line = String::new();
                        for _ in 0..config.lines {
                            let bytes = file.read_line(&mut line)?;
                            if bytes == 0 {
                                break;
                            }
                            print!("{}", line);
                            line.clear();
                        }
                        // for (line_num, line_result) in file.read_line().enumerate() {
                            
                        //     if line_num == config.lines {
                        //         break;
                        //     }
                        //     let line = line_result?;
                        //     println!("{line}");
                        // }
                    }
                    Some(_) => {
                        // for (byte_num, byte_result) in file.bytes().enumerate() {
                        //     let byte = vec![byte_result?];
                        //     if byte_num == config.bytes.unwrap() {
                        //         print!("");
                        //         break;
                        //     }
                        //     let byte = String::from_utf8_lossy(&byte);
                        //     print!("{byte}");
                        // }
                        
                    }
                }
            },
        }
    }
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    // 3は正の整数なのでOK
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // 数字でない文字列の場合はエラー
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // 0の場合もエラー
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
