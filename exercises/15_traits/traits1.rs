// The trait `AppendBar` has only one function which appends "Bar" to any object
// implementing this trait.
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    fn append_bar(self) -> String {
        let temp: String = self + "Bar"; // 也可以用format, String是有用所有权的动态字符串, 所以可以执行拼接
        temp
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
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
        assert_eq!(String::from("").append_bar().append_bar(), "BarBar");
    }
}
