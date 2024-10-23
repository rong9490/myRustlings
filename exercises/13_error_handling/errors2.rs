use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee: i32 = 1;
    let cost_per_item: i32 = 5;

    // parse 类型转换为i32, 转换失败会抛出 ParseIntError
    let qty: Result<i32, ParseIntError> = item_quantity.parse::<i32>();

    // 01. 完整匹配错误
    // let qty = match qty {
    //     Ok(q) => q, // 如果成功原样返回
    //     Err(e) => return Err(e), // 如果错误就提前结束抛出错误了~~
    // };

    // 02. 简写语法糖: 问号表达式, 向上传播错误
    let qty: i32 = qty?;

    Result::Ok(qty * cost_per_item + processing_fee)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn item_quantity_is_a_valid_number() {
        let result = total_cost("34");
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.is_err(), false);
        assert_eq!(result, Ok(171));
        assert_eq!(result.expect("error"), 171);
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        let result: Result<i32, ParseIntError> = total_cost("beep boop");
        assert_eq!(result.is_err(), true);
        assert_eq!(result.is_ok(), false);

        // 错误类型是 ParseIntError, InvalidDigit 是错误值
        let err: ParseIntError = result.unwrap_err();
        assert_eq!(err.kind(), &IntErrorKind::InvalidDigit);
        assert_eq!(err.to_string(), "invalid digit found in string");
    }
}
