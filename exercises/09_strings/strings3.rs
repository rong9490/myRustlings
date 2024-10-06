fn trim_me(input: &str) -> &str {
    // 只读的视图, 起始位置, 长度
    // trim刚好是移动两端指针起始变化, 长度变化
    // 所以trim在&str上, 方法返回的也是&str
    input.trim()
}

fn compose_me(input: &str) -> String {
    // String是可变的堆内存字符粗, 所以可以进行push操作
    let mut s: String = input.to_string();
    s.push_str(" world!");
    s
}

fn replace_me(input: &str) -> String {
    // replace也是需要修改内存的, 不能通过指针操作完成
    // 所以返回的是String类型字符串
    // 需要动态修改堆内存上的字符串, 所以返回的是String类型
    input.replace("cars", "balloons")
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
