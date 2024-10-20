fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target: &str = "rustlings";

        // 复习下: 为什么&str内存固定为16字节
        assert_eq!(std::mem::size_of::<Option<&str>>(), 16);

        let optional_target: Option<&str> = Some(target);

        // 这里Option的内存占用不变 !! None 不占用内存
        assert_eq!(std::mem::size_of::<Option<&str>>(), 16);

        // 简写的模式匹配! if let some 表达式
        let word: &str = if let Some(word) = optional_target {
            word
        } else {
            panic!("word is None");
        };

        assert_eq!(word, target);
    }

    #[test]
    fn layered_option() {
        let range: i8 = 10;

        assert_eq!(std::mem::size_of::<Option<i8>>(), 2);

        // 存None的容器
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        // 遍历range, 填充容器
        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        // TODO 待测试
        // optional_integers.push(None);

        // 游标
        let mut cursor: i8 = range;

        // 注意这里是两层 Option<Options<i8>> --> 需要两层Some(Some(integer))
        // while, 两层有一层不存在就会中断, 特别注意!! (模式匹配, 位置决定)
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        // 检查游标
        assert_eq!(cursor, 0);
    }
}
