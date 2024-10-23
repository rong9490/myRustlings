// TODO: Fix the compiler error.
fn main() {
    let mut x: i32 = 3; // 可变变量, 或者shadowing
    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");
}
