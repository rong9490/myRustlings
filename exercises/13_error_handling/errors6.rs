use std::num::ParseIntError;

#[derive(PartialEq, Debug)]
pub enum CreationError {
    Negative,
    Zero,
}

// HACK 统一的错误枚举, 涵盖群补错误分支
#[derive(PartialEq, Debug)]
pub enum ParsePosNonzeroError {
    Creation(CreationError), // 我们的枚举项包含状态(默认的error)
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    // 两个错误转换的情况, 统一成我们自定义的错误
    pub fn from_creation(err: CreationError) -> Self {
        Self::Creation(err)
    }

    // 将ParseIntError包装一层
    pub fn from_parse_int(err: ParseIntError) -> Self {
        ParsePosNonzeroError::ParseInt(err)
    }
}

#[derive(PartialEq, Debug)]
pub struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    pub fn new(value: i64) -> Result<Self, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(Self(x as u64)),
        }
    }

    pub fn parse(s: &str) -> Result<Self, ParsePosNonzeroError> {
        // HACK map_error 用法, 将默认错误转成我们特定的错误
        let x: i64 = s
            .parse()
            .map_err(|e: ParseIntError| ParsePosNonzeroError::from_parse_int(e))?;

        // impl From for ParsePosNonzeroError: 更常见的做法, 自动做类型转换 into + ?
        Self::new(x).map_err(ParsePosNonzeroError::from_creation)
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        // 结构体 + 错误枚举
        let position: Result<PositiveNonzeroInteger, ParsePosNonzeroError> =
            PositiveNonzeroInteger::parse("not a number");

        assert_eq!(std::mem::size_of::<ParsePosNonzeroError>(), 2);
        assert_eq!(std::mem::size_of_val(&position), 16);

        assert_eq!(position.is_err(), true);
        assert_eq!(position.is_ok(), false);

        // 匹配错误类型
        let matched: bool = matches!(position, Err(ParsePosNonzeroError::ParseInt(_)));
        assert_eq!(matched, true);
    }

    #[test]
    fn test_negative() {
        // 结构体 + 错误枚举
        let position: Result<PositiveNonzeroInteger, ParsePosNonzeroError> =
            PositiveNonzeroInteger::parse("-555");

        let matched: bool = matches!(
            position,
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
        assert_eq!(matched, true);

        assert_eq!(
            position,
            Err(ParsePosNonzeroError::Creation(CreationError::Negative)),
        );
    }

    #[test]
    fn test_zero() {
        let position: Result<PositiveNonzeroInteger, ParsePosNonzeroError> =
            PositiveNonzeroInteger::parse("0");

        let matched: bool = matches!(
            position,
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
        assert_eq!(matched, true);

        assert_eq!(
            position,
            Err(ParsePosNonzeroError::Creation(CreationError::Zero)),
        );
    }

    #[test]
    fn test_positive() {
        let x: PositiveNonzeroInteger = PositiveNonzeroInteger::new(42).unwrap();
        assert_eq!(x.0, 42);
        assert_eq!(PositiveNonzeroInteger::parse("42"), Ok(x));
    }
}
