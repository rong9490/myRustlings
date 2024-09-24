fn factorial(num: u64) -> u64 {
    // TODO: Complete this function to return the factorial of `num` which is
    // defined as `1 * 2 * 3 * … * num`.
    // https://en.wikipedia.org/wiki/Factorial
    //
    // Do not use:
    // - early returns (using the `return` keyword explicitly)
    // Try not to use:
    // - imperative style loops (for/while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion

    // 不要用return, 不要用循环这种命令式, 不要增加额外变量, 不要递归

    // 递归做法
    // fn do_factorial(acc: u64, num: u64) -> u64 {
    //     match num {
    //         0 | 1 => acc,
    //         _ => do_factorial(acc * num, num - 1)
    //     }
    // }

    // do_factorial(1, num)

    // iteracor 的递归 fold
    (0..=num).fold(1, |acc, num| {
        match num {
            0 | 1 => acc,
            _ => acc * num,
        }
    })

}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
