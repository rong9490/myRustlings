fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

// &String --> &str
fn main() {
    // 拥有所有权
    let word: String = String::from("green"); // Don't change this line.
    // &String --> &str 自动转换, 隐式转换;
    // 自动解引用 Deref, 
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}
