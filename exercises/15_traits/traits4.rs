trait Licensed {
    fn licensing_info(&self) -> String {
        "Default license".to_string()
    }
}

struct SomeSoftware;
struct OtherSoftware;

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// 动态分发 Box<dyn T>
// trait Object 运行时具体查找

// 两个参数满足Licensed这个Trait既可, 不需要固定是哪个具体的类型
// 两者可以交还未知, 都没有关系, 所以是: 泛型 + 泛型约束(Trait)
// 也叫静态分发: a: impl Licensed 编译时展开生成, 单体函数
// fn compare_license_types<T: Licensed, U: Licensed>(software1: U, software2: T) -> bool {
fn compare_license_types(software1: impl Licensed, software2: impl Licensed) -> bool {
    software1.licensing_info() == software2.licensing_info()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        assert!(compare_license_types(SomeSoftware, OtherSoftware));
    }

    #[test]
    fn compare_license_information_backwards() {
        assert!(compare_license_types(OtherSoftware, SomeSoftware));
    }
}
