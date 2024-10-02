// Type casting in Rust is done via the usage of the `as` operator.
// Note that the `as` operator is not only used when type casting. It also helps
// with renaming imports.

// 函数: 切片求平均值
fn average(values: &[f64]) -> f64 {
    let total: f64 = values.iter().sum::<f64>();
    let len: usize = values.len();
    let len: f64 = len as f64; // 显式的转换(安全的)
    total / len // 两个数字类型不相同
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
