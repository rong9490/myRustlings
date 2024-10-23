fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // 订单信息结构体
    #[derive(Debug)]
    struct Order {
        name: String,
        year: u32,
        made_by_phone: bool,
        made_by_mobile: bool,
        made_by_email: bool,
        item_number: u32,
        count: u32,
    }

    // 创建订单的模板工厂函数(简化, 默认值)
    fn create_order_template() -> Order {
        Order {
            name: String::from("Bob"),
            year: 2019,
            made_by_phone: false,
            made_by_mobile: false,
            made_by_email: true,
            item_number: 123,
            count: 0,
        }
    }

    #[test]
    fn your_order() {
        let order_template: Order = create_order_template();

        // 技巧(类型结构体解构): 复用原有的部分字段, 覆盖新的
        let your_order: Order = Order {
            name: "Hacker in Rust".to_string(),
            count: 1,
            ..order_template
        };

        assert_eq!(
            std::mem::size_of_val(&your_order),
            std::mem::size_of_val(&order_template)
        );

        // HACK 内存占用为40字节的原因:
        // 1. String (name): 24字节 (指针8字节, 长度8字节, 容量8字节)
        // 2. u32 (year): 4字节
        // 3. bool (made_by_phone): 1字节
        // 4. bool (made_by_mobile): 1字节
        // 5. bool (made_by_email): 1字节
        // 6. u32 (item_number): 4字节
        // 7. u32 (count): 4字节
        // 总计: 24 + 4 + 1 + 1 + 1 + 4 + 4 = 39字节
        // 由于内存对齐, 实际占用40字节
        assert_eq!(std::mem::size_of_val(&your_order), 40);

        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year);
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1);
    }
}
