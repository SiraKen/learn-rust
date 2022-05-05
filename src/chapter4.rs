/**
 * 所有権
 *
 * FIXME: 概念が難しすぎる
 * https://doc.rust-jp.rs/book-ja/ch04-01-what-is-ownership.html
 */
fn main() {
    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1);

    let a = "a";
    let b = a;

    println!("{}, world!", a);
}
