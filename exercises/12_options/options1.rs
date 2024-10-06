// This function returns how much icecream there is left in the fridge.
// If it's before 22:00 (24-hour system), then 5 scoops are left. At 22:00,
// someone eats it all, so no icecream is left (value 0). Return `None` if
// `hour_of_day` is higher than 23.

// 包含None的值枚举类型, 正常数值需要用Some包裹
fn maybe_icecream(hour_of_day: u16) -> Option<u16> {
    if hour_of_day <= 18 {
        Some(5)
    } else if hour_of_day < 24 {
        Some(0)
    } else {
        None
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

        // 偷懒写法: unwrap
        // let icecreams: u16 = icecreams.unwrap(); // 直接抛出panic!

        // 完整写法: 枚举的模式匹配
        // let icecreams = match icecreams {
        //     Some(value) => value,
        //     None => panic!(),
        // };

        // 简写: if let
        let icecreams: u16 = if let Some(icecreams) = icecreams {
            icecreams
        } else {
            panic!();
        };
        assert_eq!(icecreams, 5); // Don't change this line.
    }

    #[test]
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
    }
}
