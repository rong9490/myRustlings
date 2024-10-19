fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // 结构体除了有字段信息
    #[derive(Debug)]
    struct Package {
        sender_country: String,
        recipient_country: String,
        weight_in_grams: u32,
    }

    // 还可以存在逻辑函数, 极大拓展了Struct的实用性, 逻辑集中组织管理;
    impl Package {
        // new 关联函数(实例化)
        fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Self {
            if weight_in_grams < 10 {
                panic!("Can't ship a package with weight below 10 grams");
            }
            Self {
                sender_country,
                recipient_country,
                weight_in_grams,
            }
        }

        fn is_international(&self) -> bool {
            // 国际邮件
            self.sender_country != self.recipient_country
        }

        fn get_fees(&self, cents_per_gram: u32) -> u32 {
            cents_per_gram * self.weight_in_grams
        }
    }

    #[test]
    fn package_size() {
        let sender_country: String = String::from("Spain");
        let recipient_country: String = String::from("Austria");
        let package: Package = Package::new(sender_country, recipient_country, 1200);
        // HACK 结构体实际占用字节数 = 所有字段占用字节数之和, 本身为52, 但是因为内存对其, 需要为8的倍数, 所以为64
        assert_eq!(std::mem::size_of_val(&package), 56);
        assert_eq!(std::mem::align_of_val(&package), 8);
    }

    #[test]
    #[should_panic] // 抛出panic错误
    fn fail_creating_weightless_package() {
        let sender_country: String = String::from("Spain");
        let recipient_country: String = String::from("Austria");
        // String(胖指针) 实际占用24字节 = 指针8字节 + 长度8字节 + 容量8字节
        assert_eq!(std::mem::size_of_val(&sender_country.clone()), 24);
        assert_eq!(std::mem::size_of_val(&recipient_country.clone()), 24);
        Package::new(sender_country, recipient_country, 5);
    }

    #[test]
    fn create_international_package() {
        // 不同国家
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        assert_eq!(std::mem::size_of_val(&sender_country), 24);
        assert_eq!(std::mem::size_of_val(&recipient_country), 24);

        let package: Package = Package::new(sender_country, recipient_country, 1200);
        assert_eq!(package.is_international(), true);
    }

    #[test]
    fn create_local_package() {
        // 相同国家
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package: Package = Package::new(sender_country, recipient_country, 1200);
        assert_eq!(package.is_international(), false);
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");
        let cents_per_gram: u32 = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        // 计算费用
        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}
