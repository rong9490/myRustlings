// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

#[allow(dead_code)]
mod delicious_snacks {
    // 导出, 转发, 重命名
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    // 嵌套模块, 在中间模块可以转发/重命名子模块
    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    );
}
