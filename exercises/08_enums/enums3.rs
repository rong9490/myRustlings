// 点的 结构体
struct Point {
    x: u64, // 8字节
    y: u64, // 8字节
}

enum Message {
    Resize { width: u64, height: u64 }, // 16字节
    Move(Point),                        // 16字节
    Echo(String),                       // 24字节 = 指针(8) + 长度(8) + 容量(8)
    ChangeColor(u8, u8, u8),            // 3字节
    Quit,                               // 1字节
                                        // 总共: 16 + 16 + 24 + 3 + 1 = 60字节
                                        // 但是实际占用内存为: 64字节, 因为内存对齐
                                        // 内存对齐: 8字节
                                        // 所以实际占用内存为: 64字节
}

struct State {
    width: u64,      // 8字节
    height: u64,     // 8字节
    position: Point, // 16字节
    message: String, // 24字节
    // RGB color composed of red, green and blue.
    color: (u8, u8, u8), // 3字节
    quit: bool,          // 1字节
                         // 总共: 8 + 8 + 16 + 24 + 3 + 1 = 60字节
                         // 但是实际占用内存为: 64字节, 因为内存对齐
                         // 内存对齐: 8字节
                         // 所以实际占用内存为: 64字节
}

// State上附加的方法
impl State {
    fn resize(&mut self, width: u64, height: u64) {
        self.width = width;
        self.height = height;
    }

    fn move_position(&mut self, point: Point) {
        self.position = point;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn change_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color = (red, green, blue);
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    // 进程, 行为, 动作执行
    fn process(&mut self, message: Message) -> () {
        // variants using the methods defined above.

        // 模式匹配 match, 列举枚举的所有可能
        match message {
            Message::Resize { width, height } => self.resize(width, height),
            Message::Move(point) => self.move_position(point),
            Message::Echo(str) => self.echo(str),
            Message::ChangeColor(r, g, b) => self.change_color(r, g, b),
            Message::Quit => self.quit(),
        };
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        // 实例化State
        let mut state = State {
            width: 0,
            height: 0,
            position: Point { x: 0, y: 0 },
            message: String::from("hello world"),
            color: (0, 0, 0),
            quit: false,
        };

        assert_eq!(std::mem::size_of_val(&Point { x: 0, y: 0 }), 16); // u64占8个字节, 所以Point占16个字节
        assert_eq!(std::mem::size_of_val(&state), 64);
        assert_eq!(std::mem::align_of_val(&state), 8); // 内存对齐

        // 调用process方法
        state.process(Message::Resize {
            width: 10,
            height: 30,
        });
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Quit);

        assert_eq!(state.width, 10);
        assert_eq!(state.height, 30);
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.message, "Hello world!");
        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.quit, true);
    }
}
