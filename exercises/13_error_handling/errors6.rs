// Using catch-all error types like `Box<dyn Error>` isn't recommended for
// library code where callers might want to make decisions based on the error
// content instead of printing it out or propagating it further. Here, we define
// a custom error type to make it possible for callers to decide what to do next
// when our function returns an error.

use std::num::ParseIntError;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// A custom error type that we will be using in `PositiveNonzeroInteger::parse`.
#[derive(PartialEq, Debug)]
// HACK 统一的, 二级的错误枚举, 涵盖所有的错误情况; 方便上层根据情况具体处理!!
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    // 两个错误转换的情况, 统一成我们自定义的错误
    fn from_creation(err: CreationError) -> Self {
        Self::Creation(err)
    }

    // TODO: Add another error conversion function here.
    // 将ParseIntError包装一层
    fn from_parse_int(err: ParseIntError) -> Self {
        ParsePosNonzeroError::ParseInt(err)
    }
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(Self(x as u64)),
        }
    }

    fn parse(s: &str) -> Result<Self, ParsePosNonzeroError> {
        // when `parse()` returns an error.
        // map_error用法 在错误时, 会将默认错误转成我们特定的错误类型!!
        let x: i64 = s
            .parse()
            .map_err(|e: ParseIntError| ParsePosNonzeroError::from_parse_int(e))?;

        // impl From for ParsePosNonzeroError: 更常见的做法, 自动做类型转换 into + ?

        Self::new(x).map_err(ParsePosNonzeroError::from_creation)
    }
}

// HACK 重新理解!!

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert!(matches!(
            PositiveNonzeroInteger::parse("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_)),
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            PositiveNonzeroInteger::parse("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative)),
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            PositiveNonzeroInteger::parse("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero)),
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42).unwrap();
        assert_eq!(x.0, 42);
        assert_eq!(PositiveNonzeroInteger::parse("42"), Ok(x));
    }
}
