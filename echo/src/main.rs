use clap::Command;

fn main() {
  // アンダースコアはRustコンパイラーに、今すぐこの変数を使うつもりはないことを伝えるもの
  // アンダースコアがないと、コンパイラーは未使用の変数であることを警告する
  let _matches = Command::new("echor")
    .version("0.1.0")
    .author("tomixy")
    .about("echo command by Rust")
    .get_matches();
}
