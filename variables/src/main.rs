fn main() {
    let x = 5;
    println!("x is {}", x);
    let x = 6;
    println!("x is {}", x);
    println!("Hello, world!");
    len();
}
fn len() {
    let heart_eyed_cat = '😻'; //ハート目の猫
    let len1 = heart_eyed_cat.len_utf8();
    let len2 = String::from(heart_eyed_cat).chars().count();
    println!("{},{}", len1, len2);
}
