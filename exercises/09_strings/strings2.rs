fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    // String拥有字符串的所有权的胖指针(24字节)
    let word: String = String::from("green");

    assert_eq!(std::mem::size_of_val(&word), 24);

    // &String 自动转换 &str
    let word: &str = &word;

    // &str 胖指针, 只有 指针 + 长度 (因为是不可变的视图, 不需要容量)
    assert_eq!(std::mem::size_of_val(&word), 16);

    if is_a_color_word(word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}
