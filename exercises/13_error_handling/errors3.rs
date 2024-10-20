use std::num::ParseIntError;

fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee: i32 = 1;
    let cost_per_item: i32 = 5;
    let qty: i32 = item_quantity.parse::<i32>()?; // 问号传播错误

    Ok(qty * cost_per_item + processing_fee)
}

// `main` function.
// 抛出错误, 向上传播, 扩散的!
fn main() -> Result<(), ParseIntError> {
    let mut tokens: i32 = 100;
    let pretend_user_input: &str = "8";

    let cost: i32 = total_cost(pretend_user_input)?; // 抛出错误, 需要main函数签名接收

    match cost {
        cost if cost > tokens => println!("You can't afford that many!"),
        cost => {
            tokens -= cost;
            println!("You now have {tokens} tokens.");
        }
    }

    Ok(())
}
