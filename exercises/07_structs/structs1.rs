fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // 经典款 结构体
    pub struct ColorRegularStruct {
        red: u8,
        green: u8,
        blue: u8,
    }

    // 元组结构体
    struct ColorTupleStruct(u8, u8, u8);

    // 空结构体
    #[derive(Debug)]
    struct UnitStruct;

    #[test]
    fn regular_structs() {
        // 实例化一个结构体, 给予初始化值
        let green: ColorRegularStruct = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(std::mem::size_of_val(&green), 3); // 因为含有3个u8, 每个u8占1个字节, 所以总共占3个字节
        assert_eq!(std::mem::align_of_val(&green), 1);
        assert_eq!(std::mem::size_of_val(&green.red), 1);
        assert_eq!(std::mem::size_of_val(&green.green), 1);
        assert_eq!(std::mem::size_of_val(&green.blue), 1);

        // 点操作符, 获取属性字段
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        let green: ColorTupleStruct = ColorTupleStruct(0, 255, 0);
        assert_eq!(std::mem::size_of_val(&green), 3); // 因为含有3个u8, 每个u8占1个字节, 所以总共占3个字节
        assert_eq!(std::mem::align_of_val(&green), 1);
        assert_eq!(std::mem::size_of_val(&green.0), 1);
        assert_eq!(std::mem::size_of_val(&green.1), 1);
        assert_eq!(std::mem::size_of_val(&green.2), 1);
        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        let unit_struct: UnitStruct = UnitStruct;
        // 空结构体, 序列化为其name: UnitStruct
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
