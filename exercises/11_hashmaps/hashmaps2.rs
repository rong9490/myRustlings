use std::collections::HashMap;

// HashKey(解决哈希冲突: Hash, PartialEq, Eq)
#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

pub fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds: [Fruit; 5] = [
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    let def_value: u32 = 1;

    // or_insert: 当没有时插入, 否则跳过 = content + insert
    // 数组显式转迭代器的三种形式: iter, iter_mut, into_iter
    for fruit in fruit_kinds.into_iter() {
        basket.entry(fruit).or_insert(def_value);
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fruit_basket() -> HashMap<Fruit, u32> {
        // pair = [(key, value); 3] 数组
        let content: [(Fruit, u32); 3] = [(Fruit::Apple, 4), (Fruit::Mango, 2), (Fruit::Lychee, 5)];

        // 通过元组形式列表来创建 HashMap::from_iter
        let hash: HashMap<Fruit, u32> = HashMap::from_iter(content);
        hash
    }

    #[test]
    fn test_given_fruits_are_not_modified() {
        let mut basket: HashMap<Fruit, u32> = get_fruit_basket();
        fruit_basket(&mut basket);
        assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
        assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
        assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);
    }

    #[test]
    fn at_least_five_types_of_fruits() {
        let mut basket: HashMap<Fruit, u32> = get_fruit_basket();
        fruit_basket(&mut basket);
        let count_fruit_kinds = basket.len();
        assert!(count_fruit_kinds >= 5);
    }

    #[test]
    fn greater_than_eleven_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count = basket.values().sum::<u32>();
        assert!(count > 11);
    }

    #[test]
    fn all_fruit_types_in_basket() {
        let fruit_kinds = [
            Fruit::Apple,
            Fruit::Banana,
            Fruit::Mango,
            Fruit::Lychee,
            Fruit::Pineapple,
        ];

        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);

        for fruit_kind in fruit_kinds {
            let Some(amount) = basket.get(&fruit_kind) else {
                panic!("Fruit kind {fruit_kind:?} was not found in basket");
            };
            assert!(*amount > 0);
        }
    }
}
