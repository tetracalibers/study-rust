use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
  files: Vec<String>,
  lines: u64,
  bytes: Option<u64>,
}

pub fn get_args() -> MyResult<Config> {
  let matches = Command::new("headr")
    .version("0.1.0")
    .author("tomixy")
    .about("head command by Rust")
    .arg(
      Arg::new("lines")
        .short('n')
        .long("lines")
        .value_name("LINES")
        .help("Number of lines")
        .value_parser(clap::value_parser!(u64).range(1..))
        .default_value("10"),
    )
    .arg(
      Arg::new("bytes")
        .short('c')
        .long("bytes")
        .value_name("BYTES")
        .conflicts_with("lines")
        .value_parser(clap::value_parser!(u64).range(1..))
        .help("Number of bytes"),
    )
    .arg(
      Arg::new("files")
        .value_name("FILE")
        .help("Input file(s)")
        .num_args(1..)
        .default_value("-"),
    )
    .get_matches();

  Ok(Config {
    files: matches.get_many("files").expect("file required").cloned().collect(),
    lines: matches.get_one("lines").cloned().unwrap(),
    bytes: matches.get_one("bytes").cloned(),
  })
}

pub fn run(config: Config) -> MyResult<()> {
  let num_files = config.files.len();

  for (file_num, filename) in config.files.iter().enumerate() {
    match open(&filename) {
      Err(err) => eprintln!("{}: {}", filename, err),
      Ok(mut file) => {
        //
        // 複数ファイル間のセパレーター
        //
        if num_files > 1 {
          println!(
            "{}==> {} <==",
            if file_num > 0 { "\n" } else { "" },
            &filename
          );
        }

        if let Some(num_bytes) = config.bytes {
          //
          // バイト読み取り
          //

          // ファイルから読み込んだバイトを保持するために、
          // ゼロで満たされた固定長num_bytesのミュータブル・バッファを作成
          let mut buffer = vec![0; num_bytes as usize];

          // ファイルハンドルから希望するバイト数をバッファに読み込む
          // bytes_readの値には、実際に読み込まれたバイト数が含まれる
          let bytes_read = file.read(&mut buffer)?;

          // 選択したバイトを文字列に変換する（有効なUTF-8でない場合もある）
          // String::from_utf8_lossyは無効なUTF-8シーケンスを未知の文字または置換文字に変換する
          // 実際に読み込まれたバイトのみを選択する範囲操作を行っている
          print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
        } else {
          //
          // 行末を保持しながら行ごとに読み込む
          //

          let mut line = String::new();
          for _ in 0..config.lines {
            let bytes = file.read_line(&mut line)?;

            // ファイルハンドルは最後に達するとゼロバイトを返すので、ループから抜け出す
            if bytes == 0 {
              break;
            }

            print!("{}", line);

            // 行バッファを空にする
            line.clear();
          }
        }
      }
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

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
  match filename {
    "-" => Ok(Box::new(BufReader::new(io::stdin()))),
    _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_positive_int() {
    // 3 is an OK integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // Any string is an error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // A zero is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
  }
}
