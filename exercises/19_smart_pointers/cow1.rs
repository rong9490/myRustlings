// This exercise explores the `Cow` (Clone-On-Write) smart pointer. It can
// enclose and provide immutable access to borrowed data and clone the data
// lazily when mutation or ownership is required. The type is designed to work
// with general borrowed data via the `Borrow` trait.

// Cow = Clone on Write 保存引用或所有权, 之间转换
// 读取: 引用
// 写入: 所有
use std::borrow::Cow;

// Cow包裹的i32切片类型
fn abs_all(input: &mut Cow<[i32]>) {
    for ind in 0..input.len() {
        let value = input[ind]; // 依次取出每个数字, 如果小于零则翻转
        if value < 0 {
            // Clones into a vector if not already owned.
            // cow.to_mut() 就是转换为拥有可变的所有权, 即将写入值, 返回的是Owned
            // to_mut两种情况: Borrowed / Owned --> Owned!
            
            // 只要执行到这一句, input 就是 Cow::Owned()
            // 否则input就是默认, 可能是Cow::Owned(), 可能是Cow::Borrowed()
            input.to_mut()[ind] = -value;
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
    fn reference_mutation() {
        // Clone occurs because `input` needs to be mutated.
        let vec = vec![-5, 0, 3];
        let vec2 = vec![5, 0, 3];
        // 包装一层Cow, 默认值 Cow::Borrowed()
        let mut input = Cow::from(&vec);

        // 执行遍历修改绝对值
        abs_all(&mut input);
        // matches宏 返回bool
        // matches!(expr, pattern) 正则模式匹配
        // 宏展开 match input { Cow::Owned(_) => true, _ => false }
        
        // 由于执行了to_mut()所以变成了 Cow::Owned()
        assert_eq!(input, Cow::from(&vec2));
        assert!(!matches!(input, Cow::Borrowed(_)));
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn reference_no_mutation() {
        // No clone occurs because `input` doesn't need to be mutated.
        let vec = vec![0, 1, 2];
        // 这里默认是 Cow::Borrowed
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        // TODO: Replace `todo!()` with `Cow::Owned(_)` or `Cow::Borrowed(_)`.
        
        // 因为不会触发to_mut()操作, 所以还是默认值 Cow::Borrowed()
        assert!(matches!(input, Cow::Borrowed(_)));
        assert!(!matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn owned_no_mutation() {
        // We can also pass `vec` without `&` so `Cow` owns it directly. In this
        // case, no mutation occurs (all numbers are already absolute) and thus
        // also no clone. But the result is still owned because it was never
        // borrowed or mutated.
        let vec = vec![0, 1, 2];
        // 这里有区别, 是否传引用&
        // 这里默认是 Cow::Owned
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: Replace `todo!()` with `Cow::Owned(_)` or `Cow::Borrowed(_)`.
        assert!(!matches!(input, Cow::Borrowed(_)));
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn owned_mutation() {
        // Of course this is also the case if a mutation does occur (not all
        // numbers are absolute). In this case, the call to `to_mut()` in the
        // `abs_all` function returns a reference to the same data as before.
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: Replace `todo!()` with `Cow::Owned(_)` or `Cow::Borrowed(_)`.
        assert!(matches!(input, Cow::Owned(_)));
    }
}
