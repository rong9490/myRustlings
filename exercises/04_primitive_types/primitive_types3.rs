fn main() {
    let a: [char; 100] = ['8';100]; // 创建Array数组, 而非Vec容器

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
