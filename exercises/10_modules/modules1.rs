// mod模块, 理解成 命名空间!
// 暴露关系: 默认从严私有

mod sausage_factory {
    // 私有的
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // 共有的
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
