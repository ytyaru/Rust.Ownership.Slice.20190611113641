/*
 * 所有権（スライス型）。
 * CreatedAt: 2019-06-11
 */
fn main() {
    println!("{}", first_word(&String::from("Hello Rust !!")));
//    let s = String::from("Hello Rust !!");
//    println!("{}", first_word(&s));
//    s.clear(); // error[E0596]: cannot borrow immutable local variable `s` as mutable
}
fn first_word(target: &String) -> &str {
    let b = target.as_bytes();
    for (i, &item) in b.iter().enumerate() {
        if item == b' ' { return &target[..i]; }
    }
    &target[..]
}
