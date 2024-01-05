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

#[derive(Debug, PartialEq)]
pub struct FileInfo {
  num_lines: usize,
  num_words: usize,
  num_bytes: usize,
  num_chars: usize,
}

pub fn run(config: Config) -> MyResult<()> {
  for filename in &config.files {
    match open(filename) {
      Err(err) => eprintln!("{}: {}", filename, err),
      Ok(file) => {
        if let Ok(info) = count(file) {
          println!(
            "{}{}{}{}{}",
            format_field(info.num_lines, config.lines),
            format_field(info.num_words, config.words),
            format_field(info.num_bytes, config.bytes),
            format_field(info.num_chars, config.chars),
            if filename == "-" {
              "".to_string()
            } else {
              format!(" {}", filename)
            }
          );
        }
      }
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

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
  let mut num_lines = 0;
  let mut num_words = 0;
  let mut num_bytes = 0;
  let mut num_chars = 0;

  let mut line = String::new();

  // 行ごとに読み込む
  loop {
    // BufRead::lines は行末を削除してしまうので、
    // 代わりに BufRead::read_line を使って各行をバッファに読み込む
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

fn format_field(value: usize, show: bool) -> String {
  if show {
    format!("{:>8}", value)
  } else {
    "".to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::{count, format_field, FileInfo};
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
    assert_eq!(info.unwrap(), expected);
  }

  #[test]
  fn test_format_field() {
    assert_eq!(format_field(1, false), "");
    assert_eq!(format_field(3, true), "       3");
    assert_eq!(format_field(10, true), "      10");
  }
}
