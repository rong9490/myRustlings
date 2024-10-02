// The `From` trait is used for value-to-value conversions. If `From` is
// implemented, an implementation of `Into` is automatically provided.
// You can read more about it in the documentation:
// https://doc.rust-lang.org/std/convert/trait.From.html

// 声明一个结构体
#[derive(Debug)]
#[warn(dead_code)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

// 默认值的方法
// We implement the Default trait to use it as a fallback when the provided
// string is not convertible into a `Person` object.
impl Default for Person {
    fn default() -> Self {
        Self {
            name: String::from("John"),
            age: 30,
        }
    }
}

// TODO: Complete this `From` implementation to be able to parse a `Person`
// out of a string in the form of "Mark,20".
// Note that you'll need to parse the age component into a `u8` with something
// like `"4".parse::<u8>()`.
//
// Steps:
// 1. Split the given string on the commas present in it.
// 2. If the split operation returns less or more than 2 elements, return the
//    default of `Person`.
// 3. Use the first element from the split operation as the name.
// 4. If the name is empty, return the default of `Person`.
// 5. Parse the second element from the split operation into a `u8` as the age.
// 6. If parsing the age fails, return the default of `Person`.

// From 是一个内置trait
// 为某个类型(如Struct)实现了from
// 就能调用实现 a -> b 的类型转换

// 相反方向是: into trait 反向类型转换

impl From<&str> for Person {
    fn from(s: &str) -> Self {
        // 处理多种情况
        if s.is_empty() {
            // 这里两种写法都行
            // Person::default()
            Default::default()
        }
        // 逗号split分割
        match s.split_once(",") {
            // name为空的情况
            Some((name, _)) if name.is_empty() => {
                Default::default()
            },
            // age转数字情况
            Some((name, age_str)) => match age_str.parse::<usize>() {
                Ok(age) => Person { name: name.to_string(), age: age as u8 },
                Err(_) => Default::default(),
            },
            None => {
                Default::default()
            }
        }

    }
}


fn main() {
    // 这两个方式相同, Person实现了from; &str就可以自动调用into转Person

    // Use the `from` function.
    let p1 = Person::from("Mark,20");
    println!("{p1:?}");

    // Since `From` is implemented for Person, we are able to use `Into`.
    let p2: Person = "Gerald,70".into();
    println!("{p2:?}");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let dp: Person = Person::default(); // 获取了默认值
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }

    #[test]
    fn test_bad_convert() {
        // let p = Person::from("");

        // into如何知道找哪一个from接口呐?
        // 实际上是通过左侧返回类型, 找到Person的From的
        let p: Person = "".into();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_good_convert() {
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }

    #[test]
    fn test_bad_age() {
        // 非法年龄, 采用默认
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        // 非法年龄, 采用默认
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        // 非法年龄, 采用默认
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        // 名字为空, 采用默认
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        // 名字为空, 采用默认
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        // 名字为空, 采用默认
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        // 因为split_once, 这个会报错!
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        // 因为split_once, 这个会报错!
        let p: Person = Person::from("Mike,32,dog");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
