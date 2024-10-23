#![allow(clippy::ptr_arg)]

// 不获取所有权(Ownership)
fn get_char(data: &str) -> char {
    data.chars().last().unwrap()
}

// 获取所有权(Ownership)
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();
    println!("{data}");
}

fn main() {
    let data: String = "Rust is great!".to_string();
    get_char(&data); // removing references (the character `&`).
    string_uppercase(data); // borrow 获取参数所有权
}
