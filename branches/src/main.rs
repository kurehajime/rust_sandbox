fn main() {
    fizz_buzz(15);
}
fn fizz_buzz(count: i32) {
    for elem in 1..count + 1 {
        let elem = if elem % 3 == 0 && elem % 5 == 0 {
            String::from("Fizz Buzz")
        } else if elem % 3 == 0 {
            String::from("Fizz")
        } else if elem % 5 == 0 {
            String::from("Buzz")
        } else {
            elem.to_string()
        };
        println!("{}", elem);
    }
}
