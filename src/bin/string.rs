#![allow(unused)]
fn main() {

    let msg: String = String ::from("Hello Rust 🃏");
    let len: usize = msg.len();
    println!("msg: {msg}");
    println!("len: {len}");


    let msg: String = String ::from("Hello Rust 🃏");
    let s: &str = &msg[0..5];
    let len: usize = s.len();

    println!("Slice: {s}");
    println!("Len {len}");

    let hello: &str = "Hello Rust";
    let s: &str = &hello[0..5];
    let len: usize = s.len();

    println!("Slice: {s}");
    println!("Len {len}");

    let s: &str = r#"
    {
        "a": 1,
        "b" {"c": 2},
        "d": 3
    }
    "#;
    println!("{s}");
    // Deref coercion 
    let msg: String = String:: from ("Hello");
    let s: &str = &msg;

    let mut msg: String = "Hello".to_string();
    msg += "!";
    println!("{msg}");


    let lang = "Rust";
    let emoji = " 🃏";
    let msg = format!("hello {} {}", lang, emoji);
    println!("{msg}");
}