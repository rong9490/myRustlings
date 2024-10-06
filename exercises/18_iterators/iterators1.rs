// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use std::slice::Iter;

    #[test]
    fn iterators() {
        let my_fav_fruits: [&str; 5] = ["banana", "custard apple", "avocado", "peach", "raspberry"];

        let mut fav_fruits_iterator: Iter<'_, &str> = my_fav_fruits.iter(); // vec.iter() 将容器转迭代器

        // next() 消费一个元素(mut), Some(T) 或 None
        assert_eq!(fav_fruits_iterator.next(), Some(&"banana"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"custard apple"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"avocado"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"peach"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"raspberry"));
        assert_eq!(fav_fruits_iterator.next(), None);
    }
}
