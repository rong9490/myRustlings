#[derive(PartialEq, Debug)]
// 自定义错误枚举, 一并统筹所有情况
pub enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
pub struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    // 关联方法
    pub fn new(value: i64) -> Result<Self, CreationError> {
        // 直接返回模式匹配的值
        match value {
            x if x < 0 => Err(CreationError::Negative), // 创建一个错误实例
            0 => Err(CreationError::Zero),
            _ => Ok(Self(value as u64)),
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
