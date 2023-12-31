use clap::{Arg, ArgAction, Command};
use std::error::Error;

// deriveマクロは、構造体を出力できるようにDebug traitを追加する
#[derive(Debug)]
pub struct Config {
  files: Vec<String>,
  number_lines: bool, // 行番号を表示するかどうかを示すブール値
  number_nonblank_lines: bool, // 空白行以外の行番号の印字を制御するブール値
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
  let matches = Command::new("echor")
    .version("0.1.0")
    .author("tomixy")
    .about("cat command by Rust")
    .arg(
      Arg::new("files")
        .value_name("FILE")
        .help("Input file(s)")
        .num_args(1..)
        .default_value("-"),
    )
    .arg(
      Arg::new("number")
        .short('n')
        .long("number")
        .help("Number lines")
        .action(ArgAction::SetTrue)
        .conflicts_with("number_nonblank"),
    )
    .arg(
      Arg::new("number_nonblank")
        .short('b')
        .long("number-nonblank")
        .help("Number non-blank lines")
        .action(ArgAction::SetTrue),
    )
    .get_matches();

  Ok(Config {
    files: matches
      .get_many("files")
      .expect("files required")
      .cloned()
      .collect(),
    number_lines: matches.get_flag("number"),
    number_nonblank_lines: matches.get_flag("number_nonblank"),
  })
}

pub fn run(config: Config) -> MyResult<()> {
  dbg!(config); // 設定を表示する
  Ok(())
}
