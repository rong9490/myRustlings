// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    string_slice("blue"); // slice是切片是, 只需要移动指针和长度既可, 所以&str

    string("red".to_string()); // &str --> String

    string(String::from("hi")); // &str --> String

    string("rust is fun!".to_owned()); // &str --> String 因为获取了所有权, 所以是拥有所有权的String

    string("nice weather".into()); // &str --> String; From 和 Into 类型转换的trait; 编译器寻找相关的into, 自动找实现

    string(format!("Interpolation {}", "Station")); // 方便拼接字符串的宏format!

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]); // String --> &String --> &str 切片语法

    string_slice("  hello there ".trim()); // 内存字符串不需要变化, 只需要移动指针和长度

    string("Happy Monday!".replace("Mon", "Tues")); // 内存字符串需要变化, 开辟新的空间, 所以返回String类型

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());  // 内存字符串需要变化, 开辟新的空间, 所以返回String类型
}
