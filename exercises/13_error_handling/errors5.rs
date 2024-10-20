use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// 实现Display trait, 打印输出
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description: &str = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl Error for CreationError {}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// use to describe both errors? Is there a trait which both errors implement?
// Box<dyn Error> 包含了 ParseIntError 和 CreationError!!
// 返回通用的错误, 不一定需要指定具体的错误类型

// 兼容多种错误类型: 错误类型的顶层 Trait: Box<dyn Error>
fn main() -> Result<(), Box<dyn Error>> {
    let pretend_user_input: &str = "42";
    let x: i64 = pretend_user_input.parse()?; // 抛出ParseIntError

    let result: PositiveNonzeroInteger = PositiveNonzeroInteger::new(x)?; // 抛出CreationError
    println!("output={:?}", result);
    Ok(())
}
