fn current_favorite_color() -> String {
    "blue".to_string()
}

fn main() {
    let answer: String = current_favorite_color();
    // 胖指针: 指针 + 长度 + 容量 = 24字节
    assert_eq!(std::mem::size_of_val(&answer), 24);
    println!("My current favorite color is {answer}");
}
