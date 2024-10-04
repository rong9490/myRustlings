fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let a: [char; 100] = ['8';100]; // 创建数组的方式, 不是vec容器

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
