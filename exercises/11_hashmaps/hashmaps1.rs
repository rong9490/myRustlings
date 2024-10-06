// A basket of fruits in the form of a hash map needs to be defined. The key
// represents the name of the fruit and the value represents how many of that
// particular fruit is in the basket. You have to put at least 3 different
// types of fruits (e.g. apple, banana, mango) in the basket and the total count
// of all the fruits should be at least 5.

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // HashMap::with_capacity 预先分配内存, 可能会更多;
    let mut basket: HashMap<String, u32> = HashMap::new();

    // Two bananas are already given for you :)
    // 哈希的key 需要获得所有权, 所以是String
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
    fn at_least_three_types_of_fruits () {
        let basket: HashMap<String, u32> = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket: HashMap<String, u32> = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5); // 迭代器 -> 适配器 --> 消费器, 闭包
    }
}
