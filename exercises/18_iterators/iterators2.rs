// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

use std::str::Chars;

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars: Chars<'_> = input.chars(); // 字符串产生的迭代器, 消费完第一个字符, 然后将剩下的重新组装回去as_str
    let first_char: Option<char> = chars.next();
    match first_char {
        None => String::new(), // 如果首个字符都没有, 那么返回空字符串
        Some(first) => {
            let rest_chars: &str = chars.as_str();
            first.to_uppercase().to_string() + rest_chars
        },
    }
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {

    // 迭代器遍历每格单词, 然后map适配器
    // 最后collect重新组装回容器, 组装成什么类型, 通过turbofish操作
    words.iter().map(|&word| {
        capitalize_first(word)
    }).collect::<Vec<String>>()
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|&word| {
        // TODO String, Vec<String>, 像魔法
        capitalize_first(word)
    }).collect::<String>()
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
