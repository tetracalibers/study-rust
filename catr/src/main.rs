fn main() {
  // catr::run関数を実行し、戻り値がErr(e)にマッチするかどうかをチェックする
  if let Err(e) = catr::get_args().and_then(catr::run) {
    // eprintln!マクロを使って、エラーメッセージをSTDERRにプリント
    eprintln!("{}", e);
    // エラーを示す非ゼロ値でプログラムを終了
    std::process::exit(1);
  }
}
