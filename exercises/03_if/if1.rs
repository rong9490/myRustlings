fn bigger(a: i32, b: i32) -> i32 {
    if a >= b {
        a
    } else {
        b
    }
}

fn main() {
    // 单元测试外也可以使用 assert_eq
    assert_eq!(42, bigger(42, 24))
}

// 条件编译: 测试模块
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
