pub fn trim_me(input: &str) -> &str {
    // trim方法 只需要改变视图指针和长度, 所以来源和返回都是 &str
    input.trim()
}

pub fn compose_me(input: &str) -> String {
    // String是 可变的 / 拥有所有权的, 堆内存字符串, 可以进行push操作
    let mut s: String = input.to_string();
    s.push_str(" world!");
    s
}

pub fn replace_me(input: &str) -> String {
    // replace需要修改内存, 不能通过指针操作完成(所以不能用&str)
    // String 可修改 / 有所有权的 字符串, 所以返回的是String类型
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
        // 消除两端空格
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
