// Option<u16> 包含None的值枚举类型, 正常数值需要用Some包裹
pub fn maybe_icecream(hour_of_day: u16) -> Option<u16> {
    if hour_of_day <= 18 {
        Option::Some(5)
    } else if hour_of_day < 24 {
        Option::Some(0)
    } else {
        Option::None
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        let icecreams: Option<u16> = maybe_icecream(12);

        // 内存占用与枚举类型有关
        assert_eq!(std::mem::size_of::<Option<u16>>(), 4);

        assert_eq!(icecreams.is_some(), true);
        assert_eq!(icecreams.is_none(), false);
        assert_eq!(icecreams.unwrap(), 5);

        // 01. 偷懒写法: unwrap
        // let icecreams: u16 = icecreams.unwrap(); // 直接抛出panic!

        // 02. 完整写法: 枚举的模式匹配
        // let icecreams = match icecreams {
        //     Some(value) => value,
        //     None => panic!(),
        // };

        // 03. 简写: if let 表达式
        let icecreams: u16 = if let Some(icecreams) = icecreams {
            icecreams
        } else {
            panic!("icecreams is None");
        };

        assert_eq!(icecreams, 5);
    }

    #[test]
    #[should_panic]
    fn check_icecream() {
        assert_eq!(maybe_icecream(0), Some(5));
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(18), Some(5));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(24), None);
        assert_eq!(maybe_icecream(25), None);

        // 两个来判断 is_none, is_some 的断言方法
        assert_eq!(maybe_icecream(25).is_none(), true);
        assert_eq!(maybe_icecream(25).is_some(), false);

        // 断言会抛出panic!
        assert_eq!(maybe_icecream(25).unwrap(), 5);
    }
}
