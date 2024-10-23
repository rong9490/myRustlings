use std::collections::HashMap; // 集合(collections)

pub fn fruit_basket() -> HashMap<String, u32> {
    // HashMap::with_capacity 预先定个容量
    let mut basket: HashMap<String, u32> = HashMap::new();

    // 哈希的key需要获得所有权, 所以是String
    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("orange"), 1);
    basket.insert(String::from("apple"), 2);
    basket
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn at_least_three_types_of_fruits() {
        let basket: HashMap<String, u32> = fruit_basket();
        assert!(basket.len() >= 3); // len表示key的数量

        assert_eq!(basket.get("banana").is_some(), true);
        assert_eq!(basket.get("banana").is_none(), false);
        assert_eq!(basket.get("banana").unwrap(), &2); // 取出的值是引用!

        assert_eq!(basket.get("orange").is_some(), true);
        assert_eq!(basket.get("orange").unwrap(), &1);

        assert_eq!(basket.get("apple").is_some(), true);
        assert_eq!(basket.get("apple").unwrap(), &2);

        assert_eq!(basket.get("xxyy").is_none(), true);
        assert_eq!(basket.get("xxyy").unwrap(), &0); // panic!
    }

    #[test]
    fn at_least_five_fruits() {
        let basket: HashMap<String, u32> = fruit_basket();
        // 哈希表(basket) -> 迭代器(values) -> 消费器(sum::<u32>()) -> 断言(assert)
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
