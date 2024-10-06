// Don't change this function.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // 两者的作用域交集需要获得足够长, 已支撑result的使用安全
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result: &str;

    // blockd 产生新作用域块, 声明周期会更短
    {
        // `string2` does not live long enough borrowed value does not live long enough
        result = longest(&string1, &string2);
    }
    println!("{}", string1);
    println!("{}", string2);
    println!("The longest string is '{result}'");
}
