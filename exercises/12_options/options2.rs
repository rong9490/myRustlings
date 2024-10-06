fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // if let Some() = 
        // 这种结构型语法, 处理Option匹配! == match
        // 简写的模式匹配!
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: Make this a while-let statement. Remember that `Vec::pop()`
        // adds another layer of `Option`. You can do nested pattern matching
        // in if-let and while-let statements.

        // 注意这里是两层 Option<Options<i8>> --> 需要两层Some(Some(integer))
        // 模式匹配, 位置决定
        // 这里 while, 两层有一层不存在就会中断, 特别注意!!
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        // 检查游标
        assert_eq!(cursor, 0);
    }
}
