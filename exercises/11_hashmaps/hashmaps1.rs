// A basket of fruits in the form of a hash map needs to be defined. The key
// represents the name of the fruit and the value represents how many of that
// particular fruit is in the basket. You have to put at least 3 different
// types of fruits (e.g. apple, banana, mango) in the basket and the total count
// of all the fruits should be at least 5.

use std::collections::HashMap;

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
    fn at_least_three_types_of_fruits() {
        let basket: HashMap<String, u32> = fruit_basket();
        assert!(basket.len() >= 3); // len表示key的数量

        assert_eq!(basket.get("banana").is_some(), true);
        assert_eq!(basket.get("banana").is_none(), false);
        assert_eq!(basket.get("orange").is_some(), true);
        assert_eq!(basket.get("apple").is_some(), true);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket: HashMap<String, u32> = fruit_basket();
        // 哈希表(basket) -> 迭代器(values) -> 消费器(sum::<u32>()) -> 断言(assert)
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
