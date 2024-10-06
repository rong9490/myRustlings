// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?

// lifetime 本质上是依赖: Outlive
// 引用, 推断出生命周期, 显式/隐式, 缺省
// 引用, 就是内存; 
// 依赖者一定先释放, 被依赖者一定后释放, 才是内存安全的!!

// 两者 生命周期较短者; 防止造成悬垂指针, 野指针,
// 显式标注生命周期是多少, 或者长短关系: Outlive
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}
