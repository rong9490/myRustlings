// TODO: Fix the compiler error by moving the whole definition of this macro.
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

// 宏必须先声明后使用, 编译器才能找到展开

fn main() {
    my_macro!();
}