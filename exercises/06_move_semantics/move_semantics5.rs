#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data: String = "Rust is great!".to_string();

    get_char(data.clone()); // move -> clone, 不获取所有权, 不影响本体data的所有权

    string_uppercase(data); // borrow 获取参数所有权
}
