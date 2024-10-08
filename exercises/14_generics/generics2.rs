// This powerful wrapper provides the ability to store a positive integer value.
struct Wrapper<T> {
    value: T,
}

// impl 可以实现泛型的某一种类型的 具体实现!
// impl Wrapper<u32>
// impl Wrapper<&str> 

impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
