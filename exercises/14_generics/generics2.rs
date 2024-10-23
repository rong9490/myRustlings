pub struct Wrapper<T> {
    pub value: T, // 结构体字段也能用泛型
}

// impl 可以为其中一种泛型实现方法!
// impl Wrapper<u32>
// impl Wrapper<&str> 

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
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
        let wrapper: Wrapper<u32> = Wrapper::new(42);

        assert_eq!(std::mem::size_of::<Wrapper<u32>>(), 4);
        assert_eq!(std::mem::size_of_val(&wrapper), 4);
        assert_eq!(std::mem::size_of_val(&wrapper.value), 4);

        assert_eq!(wrapper.value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        let wrapper: Wrapper<&str> = Wrapper::new("Foo");

        assert_eq!(std::mem::size_of::<Wrapper<&str>>(), 16);
        assert_eq!(std::mem::size_of_val(&wrapper), 16);
        assert_eq!(std::mem::size_of_val(&wrapper.value), 16);

        assert_eq!(wrapper.value, "Foo");
    }
}
