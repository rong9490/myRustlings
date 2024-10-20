fn main() {
    // 容器只能放置相同类型的数据 i32
    let mut numbers: Vec<i32> = Vec::new();

    // 胖指针内存固定24
    assert_eq!(std::mem::size_of::<Vec<i32>>(), 24);

    let n1: u8 = 42;
    let n1_into: i32 = n1.into(); // core::convert From trait
    numbers.push(n1_into);

    let n2: i8 = -1;
    let n2_into: i32 = n2.into();
    numbers.push(n2_into);

    println!("{numbers:?}");

    assert_eq!(numbers.len(), 2);
    assert_eq!(std::mem::size_of::<Vec<i32>>(), 24);
}
