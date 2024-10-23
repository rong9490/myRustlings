// 声明一个trait, 里面有一个函数签名(没有默认实现) fn append_bar(self) -> Self;
trait AppendBar {
    fn append_bar(self) -> Self;
}

// 为String类型实现AppendBar trait
// 这样所有String类型的字符串都可以调用该方法
impl AppendBar for String {
    fn append_bar(self: String) -> String {
        let temp: String = self + "Bar"; // 也可以用format, String有所有权的动态字符串, 可以执行拼接
        temp
    }
}

fn main() {
    let s: String = String::from("Foo");
    let s: String = s.append_bar();
    println!("s: {s}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), "FooBar");
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("")
                .append_bar()
                .append_bar()
                .append_bar()
                .append_bar(),
            "BarBarBarBar"
        );
    }
}
