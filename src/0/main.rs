/*
 * 所有権（スライス型）。
 * CreatedAt: 2019-06-11
 */
fn main() {
    println!("{}", first_word(String::from("Hello Rust !!")));
}
fn first_word(target: String) -> usize {
    let b = target.as_bytes();
    for (i, &item) in b.iter().enumerate() {
        if item == b' ' { return i; }
    }
    target.len()
}
