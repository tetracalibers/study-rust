use assert_cmd::Command;
use predicates::prelude::*;

// 引数を指定しない場合、失敗してヘルプ・ドキュメントを表示する
#[test]
fn dies_no_args() {
  let mut cmd = Command::cargo_bin("echor").unwrap();
  // 引数なしでプログラムを実行 => 失敗 => STDERRに使用法を表示
  cmd.assert().failure().stderr(predicate::str::contains("Usage"));
}

// 引数が与えられたときにプログラムが正常に終了する
#[test]
fn runs() {
  let mut cmd = Command::cargo_bin("echor").unwrap();
  cmd.arg("hello").assert().success();
}
