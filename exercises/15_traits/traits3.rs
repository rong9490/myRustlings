#![allow(dead_code)]

trait Licensed {
    // Trait的默认实现
    fn licensing_info(&self) -> String {
        "Default license".to_string()
    }
}

// '软件1' 结构体
struct SomeSoftware {
    version_number: i32,
}

// '软件2' 结构体
struct OtherSoftware {
    version_number: String,
}

// 都实现Licensed Trait(采用默认实现)
impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        // 两个结构体实例
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };

        // 调用Trait的默认实现
        assert_eq!(
            some_software.licensing_info(),
            other_software.licensing_info()
        );
    }
}
