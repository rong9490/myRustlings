#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point: Point = Point { x: 100, y: 200 };

    // 理解这里的内存: size_of(编译时) size_of_val(运行时)
    assert_eq!(std::mem::size_of::<Point>(), 8); // 编译时
    assert_eq!(std::mem::size_of_val(&point), 8); // 运行时
    assert_eq!(std::mem::size_of_val(&point.x), 4); // 运行时
    assert_eq!(std::mem::size_of_val(&point.y), 4); // 运行时

    let optional_point: Option<Point> = Some(point);

    // 结构体 + 判别式 + 内存对齐 = 12
    assert_eq!(std::mem::size_of::<Option<Point>>(), 12);

    // HACK Some(ref p) 什么含义
    // 匹配引用而不拿所有权, 只读操作是没有问题
    match optional_point {
        Some(ref p) => println!("Co-ordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
    };

    // match只获取引用, 所以这里依然所有权有效
    println!("{optional_point:?}");

    assert_eq!(optional_point.is_some(), true);
    assert_eq!(optional_point.is_none(), false);

    assert_eq!(std::mem::size_of::<Option<Point>>(), 12);
}
