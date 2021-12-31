use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("数当てゲーム❗");

    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("数字を入れてね");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("読み取り失敗！");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("あなたの予想は {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さいよ"),
            Ordering::Equal => {
                println!("ちょうど");
                break;
            }
            Ordering::Greater => println!("大きいよ"),
        }
    }
}
