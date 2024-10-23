// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

use std::str::Chars;

// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    // 字符串产生的迭代器
    let mut chars: Chars<'_> = input.chars();

    // 消费完第一个字符, 然后将剩下的重新组装回去as_str
    let first_char: Option<char> = chars.next();
    match first_char {
        None => String::new(), // 如果首个字符都没有, 那么返回空字符串
        Some(first) => {
            // 拿到剩下的字符迭代器, 重新组装回字符串
            let rest_chars: &str = chars.as_str();
            // 然后首字母大写, 拼接上剩余字符串
            first.to_uppercase().to_string() + rest_chars
        }
    }
}

// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // 迭代器遍历每格单词, 然后map适配器
    // 最后collect重新组装回容器, 组装成什么类型, 通过turbofish操作
    // 遍历容器, 使其每一项调用上面的函数
    // iter().map(|| {}) 迭代器, 适配器, 闭包
    words
        .iter()
        .map(|&word| capitalize_first(word))
        .collect::<Vec<String>>()
}

// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    words
        .iter()
        .map(|&word| capitalize_first(word))
        // 最后多了一步, 重新收集回来
        // 会根据类型自动调用对应的collect方法 turbofish 写法
        .collect::<String>()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
