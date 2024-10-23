// TODO: Fix the compiler error by adding one or two characters.
#[rustfmt::skip]
macro_rules! my_macro {
    // 模式匹配1: 无参数
    () => {
        println!("Check out my macro!");
    };
    // 模式匹配2: 一个参数
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
