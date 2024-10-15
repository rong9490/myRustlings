fn main() {
    // tuple 元组
    let cat: (&str, f64) = ("Furry McFurson", 3.5);

    assert_eq!(cat.0.len(), 14);
    assert_eq!(std::mem::size_of_val(cat.0), 14);
    assert_eq!(cat.0.len(), std::mem::size_of_val(cat.0)); // 为什么两者相等
    println!("{}", dbg!(cat.0));

    // 模式匹配, 解构
    let (name, age) = cat;
    assert_eq!(age, 3.5);
    assert_eq!(std::mem::size_of_val(&age), 8); // float占8个字节
    println!("{name} is {age} years old");
}
