// @see https://doc.rust-jp.rs/book-ja/ch02-00-guessing-game-tutorial.html

use rand::Rng;
use std::io;

fn main() {
  println!("Guess the number!"); // 数を当ててごらん

  let secret_number = rand::thread_rng().gen_range(1..101);

  println!("The secret number is: {}", secret_number); //秘密の数字は次の通り: {}

  println!("Please input your guess."); // ほら、予想を入力してね

  let mut guess = String::new();

  io::stdin().read_line(&mut guess).expect("Failed to read line"); // 行の読み込みに失敗しました

  println!("You guessed: {}", guess); // 次のように予想しました: {}
}
