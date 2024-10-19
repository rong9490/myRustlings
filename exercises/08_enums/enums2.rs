#![allow(dead_code)]

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

// 枚举的值可以有非常多样, 不同类型, 且可以带'状态'
#[derive(Debug)]
enum Message {
    Resize { width: u8, height: u8 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages: [Message; 5] = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_size() {
        let p: Point = Point { x: 10, y: 15 };
        assert_eq!(std::mem::size_of_val(&p.x), 8);
        assert_eq!(std::mem::size_of_val(&p.y), 8);
        // 结构体内存 = 各字段内存之和
        assert_eq!(std::mem::size_of_val(&p), 16);
    }

    #[test]
    fn test_message_size() {
        // 整个枚举的内存由最大那个决定, 就是固定的24
        assert_eq!(std::mem::size_of::<Message>(), 24);

        assert_eq!(
            std::mem::size_of_val(&Message::Resize {
                width: 0,
                height: 0
            }),
            24
        );
        assert_eq!(
            std::mem::size_of_val(&Message::Move(Point { x: 0, y: 0 })),
            24
        );

        assert_eq!(std::mem::size_of_val(&Message::Echo(String::from(""))), 24);

        assert_eq!(std::mem::size_of_val(&Message::ChangeColor(0, 0, 0)), 24);

        // 枚举内存是由最大那个项决定的! 即使Quit为0
        assert_eq!(std::mem::size_of_val(&Message::Quit), 24);
    }
}
