// 三种类型的错误枚举
#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    // Example: 42 / 0
    DivideByZero,
    // Only case for `i64`: `i64::MIN / -1` because the result is `i64::MAX + 1`
    IntegerOverflow,
    // Example: 5 / 2 = 2.5
    NotDivisible,
}

/* HACK 没看懂! 后续多理解几遍 */

// TODO: Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    // 元组看成一个整体来进行模式匹配
    match (a, b) {
        (i64::MIN, -1) => Err(DivisionError::IntegerOverflow),
        (_, 0) => Err(DivisionError::DivideByZero), // 除0错误
        (a, b) if a % b != 0 => Err(DivisionError::NotDivisible), // 不能除尽错误
        _ => Ok(a / b),
    }
}

// TODO: Add the correct return type and complete the function body.
// Desired output: `Ok([1, 11, 1426, 3])`
fn result_with_list() -> Result<[i64; 4], DivisionError> {
    let numbers: [i64; 4] = [27, 297, 38502, 81];
    let division_results: Result<Vec<i64>, DivisionError> =
        numbers.into_iter().map(|n| divide(n, 27)).collect();
    division_results.map(|v| [v[0], v[1], v[2], v[3]])
}

// TODO: Add the correct return type and complete the function body.
// Desired output: `[Ok(1), Ok(11), Ok(1426), Ok(3)]`
fn list_of_results() -> Vec<Result<i64, DivisionError>> {
    let numbers = [27, 297, 38502, 81];
    numbers
        .into_iter()
        .map(|n| divide(n, 27))
        .collect::<Vec<Result<i64, DivisionError>>>()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_integer_overflow() {
        assert_eq!(divide(i64::MIN, -1), Err(DivisionError::IntegerOverflow));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(divide(81, 6), Err(DivisionError::NotDivisible));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(result_with_list().unwrap(), [1, 11, 1426, 3]);
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(list_of_results(), [Ok(1), Ok(11), Ok(1426), Ok(3)]);
    }
}
