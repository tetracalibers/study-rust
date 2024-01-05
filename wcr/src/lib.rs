use clap::{Arg, ArgAction, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
  files: Vec<String>,
  lines: bool,
  words: bool,
  bytes: bool,
  chars: bool,
}

pub fn run(config: Config) -> MyResult<()> {
  for filename in &config.files {
    match open(filename) {
      Err(err) => eprintln!("{}: {}", filename, err),
      Ok(_) => println!("Opened {}", filename),
    }
  }

  Ok(())
}

pub fn get_args() -> MyResult<Config> {
  let matches = Command::new("wcr")
    .version("0.1.0")
    .author("tomixy")
    .about("wc command by Rust")
    .arg(
      Arg::new("files")
        .value_name("FILE")
        .help("Input file(s)")
        .num_args(1..)
        .default_value("-"),
    )
    .arg(
      Arg::new("bytes")
        .short('c')
        .long("bytes")
        .value_name("BYTES")
        .help("Show byte count")
        .action(ArgAction::SetTrue),
    )
    .arg(
      Arg::new("chars")
        .short('m')
        .long("chars")
        .value_name("CHARS")
        .help("Show character count")
        .action(ArgAction::SetTrue)
        .conflicts_with("bytes"),
    )
    .arg(
      Arg::new("lines")
        .short('l')
        .long("lines")
        .value_name("LINES")
        .help("Show line count")
        .action(ArgAction::SetTrue),
    )
    .arg(
      Arg::new("words")
        .short('w')
        .long("words")
        .value_name("WORDS")
        .help("Show word count")
        .action(ArgAction::SetTrue),
    )
    .get_matches();

  let mut lines = matches.get_flag("lines");
  let mut words = matches.get_flag("words");
  let mut bytes = matches.get_flag("bytes");
  let chars = matches.get_flag("chars");

  if [words, bytes, chars, lines].iter().all(|v| v == &false) {
    lines = true;
    words = true;
    bytes = true;
  }

  Ok(Config {
    files: matches
      .get_many("files")
      .expect("files required")
      .cloned()
      .collect(),
    lines,
    words,
    bytes,
    chars,
  })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
  match filename {
    "-" => Ok(Box::new(BufReader::new(io::stdin()))),
    _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
  }
}
