// TODO: Fix the compiler error without taking the macro definition out of this
// module.
#[macro_use]
mod macros {
    #[macro_export] macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
    
    // pub(crate) use my_macro;
}

fn main() {
    my_macro!();
}

// 知识点1 #[macro_use]
// 知识点2 #[macro_export]
// 知识点3 pub(crate) use my_macro;