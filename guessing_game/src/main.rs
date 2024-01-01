// @see https://doc.rust-jp.rs/book-ja/ch02-00-guessing-game-tutorial.html

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  println!("Guess the number!"); // 数を当ててごらん

  let secret_number = rand::thread_rng().gen_range(1..101);

  println!("The secret number is: {}", secret_number); //秘密の数字は次の通り: {}

  loop {
    println!("Please input your guess."); // ほら、予想を入力してね

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line"); // 行の読み込みに失敗しました

    let guess: u32 = guess.trim().parse().expect("Please type a number!"); //数値を入力してください！

    println!("You guessed: {}", guess); // 次のように予想しました: {}

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"), //小さすぎ！
      Ordering::Greater => println!("Too big!"), //大きすぎ！
      Ordering::Equal => println!("You win!"),  //やったね！
    }
  }
}
