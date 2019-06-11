/*
 * 所有権（スライス型）。
 * CreatedAt: 2019-06-11
 */
fn main() {
//    println!("{}", first_word(&String::from("Hello Rust !!")));
    let s = String::from("Hello Rust !!");
    println!("{}", first_word(&s)); // これでもいけてしまう
    println!("{}", first_word(&s[..])); // Stringから&str型へ
    let l = "ABC DEF";
    println!("{}", first_word(&l)); // lはすでに文字列リテラルなので[..]不要
    println!("{}", first_word(&l[..])); // [..]があってもOK
}
//fn first_word(target: &String) -> &str {
fn first_word(target: &str) -> &str {
    let b = target.as_bytes();
    for (i, &item) in b.iter().enumerate() {
        if item == b' ' { return &target[..i]; }
    }
    &target[..]
}
