use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// 引数を指定しない場合、失敗してヘルプ・ドキュメントを表示する
#[test]
fn dies_no_args() -> TestResult {
  let mut cmd = Command::cargo_bin("echor")?;
  cmd.assert().failure().stderr(predicate::str::contains("Usage"));
  // これまでのコードがすべて機能した場合は、ユニット・タイプを含むOkを返す
  // この値を返すには、最後のセミコロンを省略する
  Ok(())
}

// argsは&str値のスライス、expect_fileは&str、戻り値はTestResult
fn run(args: &[&str], expected_file: &str) -> TestResult {
  // expected_fileの内容を文字列に読み込もうとする
  let expected = fs::read_to_string(expected_file)?;
  Command::cargo_bin("echor")?.args(args).assert().success().stdout(expected);
  Ok(())
}

#[test]
fn hello1() -> TestResult {
  // この関数は、run関数が返すものなら何でも返すので、終端のセミコロンがないことに注意
  run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
  run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
  run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
  run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
