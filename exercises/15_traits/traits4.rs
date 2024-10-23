pub trait Licensed {
    fn licensing_info(&self) -> String {
        "Default license".to_string()
    }
}

pub struct SomeSoftware;
pub struct OtherSoftware;

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}


// 相当于 TS 中的 鸭子类型
// '静态分发' --> 编译阶段生成多种具体类型
// '动态分发' --> Box<dyn T> 运行时具体查找
// fn compare_license_types<T: Licensed, U: Licensed>(software1: U, software2: T) -> bool {
pub fn compare_license_types(software1: impl Licensed, software2: impl Licensed) -> bool {
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
