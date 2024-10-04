// 经典款 结构体
struct ColorRegularStruct {
    red: u8,
    green: u8,
    blue: u8,
    // What types should the fields have? What are the minimum and maximum values for RGB colors?
}

// 元组结构体
struct ColorTupleStruct(
    u8,
    u8,
    u8,
);

// 空结构体
#[derive(Debug)]
struct UnitStruct;

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // 实例化一个结构体, 给予初始化值
        let green: ColorRegularStruct = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        // 点操作符, 获取属性字段
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        let green: ColorTupleStruct = ColorTupleStruct (
            0, 255, 0,
        );

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
