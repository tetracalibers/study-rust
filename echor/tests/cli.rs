use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// 引数を指定しない場合、失敗してヘルプ・ドキュメントを表示する
#[test]
fn dies_no_args() -> TestResult {
  let mut cmd = Command::cargo_bin("echor")?;
  cmd.assert().failure().stderr(predicate::str::contains("Usage"));
  // この値を返すには、最後のセミコロンを省略する
  Ok(())
}

// 引数が与えられたときにプログラムが正常に終了する
#[test]
fn runs() {
  let mut cmd = Command::cargo_bin("echor").unwrap();
  cmd.arg("hello").assert().success();
}

#[test]
fn hello1() -> TestResult {
  let expected = fs::read_to_string("tests/expected/hello1.txt")?;
  let mut cmd = Command::cargo_bin("echor")?;
  cmd.arg("Hello there").assert().success().stdout(expected);
  Ok(())
}

#[test]
fn hello2() -> TestResult {
  let expected = fs::read_to_string("tests/expected/hello2.txt")?;
  let mut cmd = Command::cargo_bin("echor")?;
  cmd.args(vec!["Hello", "there"]).assert().success().stdout(expected);
  Ok(())
}
