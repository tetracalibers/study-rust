use std::error::Error;

// deriveマクロは、構造体を出力できるようにDebug traitを追加する
#[derive(Debug)]
pub struct Config {
  files: Vec<String>,
  number_lines: bool, // 行番号を表示するかどうかを示すブール値
  number_nonblank_lines: bool, // 空白行以外の行番号の印字を制御するブール値
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
  println!("Hello, world!");
  Ok(())
}
