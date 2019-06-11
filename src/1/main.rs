/*
 * [文字列スライス](https://doc.rust-jp.rs/book/second-edition/ch04-03-slices.html#a%E6%96%87%E5%AD%97%E5%88%97%E3%82%B9%E3%83%A9%E3%82%A4%E3%82%B9)
 * CreatedAt: 2019-06-11
 */
fn main() {
    let s = String::from("Hello world"); // String型
    let hello = &s[0..5];  // &str型 [..5]
    let world = &s[6..11]; // &str型 [6..] [6..len]
    println!("{}", s);
    println!("{}", hello);
    println!("{}", world);
}

