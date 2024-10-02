macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

// 声明宏, 如: matches!() println!()
// 过程宏, 如: #[cfg(test)] #[derive(Debug)]

fn main() {
    // TODO: Fix the macro call.
    my_macro!();
}
