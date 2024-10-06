#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point: Option<Point> = Some(Point { x: 100, y: 200 });


    // HACK Some(ref p) 什么含义
    // 匹配引用, 而不是拿到所有权, 只是读的操作是没有问题的
    match optional_point {
        Some(ref p) => println!("Co-ordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
    };

    // borrow of partially moved value: `optional_point`
    // 因为point有部分所有权已经转给了match, 使用到了x,y; 在后续再用到了, 报错!
    // match会拿到匹配者的所有权!! 如何解决 主动加个ref关键字, 因为只是读取操作不会有问题!
    println!("{optional_point:?}"); // Don't change this line.
}
