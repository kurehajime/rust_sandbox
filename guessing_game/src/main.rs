use std::io;

fn main() {
    println!("数当てゲーム❗");
    println!("数字を入れてね");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("読み取り失敗！");

    println!("あなたの予想は {}",guess);
}
