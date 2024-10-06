fn current_favorite_color() -> String {
    "blue".to_string()
}

fn main() {
    // 拥有所有权的String, 分配在堆上;
    // &str 是不可变的字符串切片, 分配在栈上; 指针 + 长度
    let answer: String = current_favorite_color();
    println!("My current favorite color is {answer}");
}
