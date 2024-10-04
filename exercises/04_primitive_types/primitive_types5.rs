fn main() {
    // tuple 元组
    let cat: (&str, f64) = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;
    // 匹配模式: 解构 / 析构
    let (name, age) = cat;

    println!("{name} is {age} years old");
}
