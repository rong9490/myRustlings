#[derive(Debug)]
enum Message {
    Resize,
    Move,
    Echo,
    ChangeColor,
    Quit,
}

// 简单枚举, 会优化表示, 2 ^ 3 = 8足够表示了, 所以一个占用字节

fn main() {
    // 枚举需要用双冒号调用字段
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);

    // 判断枚举的内存大小
    println!("枚举 Message 的内存大小为: {} 字节", std::mem::size_of::<Message>());
    // 判断枚举的对齐大小
    println!("枚举 Message 的对齐大小为: {} 字节", std::mem::align_of::<Message>());

    // 全部都是1
    assert_eq!(std::mem::size_of_val(&Message::Quit), 1);
    assert_eq!(std::mem::size_of_val(&Message::Resize), 1);
    assert_eq!(std::mem::size_of_val(&Message::Move), 1);
    assert_eq!(std::mem::size_of_val(&Message::Echo), 1);
    assert_eq!(std::mem::size_of_val(&Message::ChangeColor), 1);
}
