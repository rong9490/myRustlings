trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Vec<String> {
        // mut Vec<String>
        // 容器的追加元素操作push
        self.push(String::from("Bar"));
        self
    }
}

// `append_bar` should push the string "Bar" into the vector.
fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo: Vec<String> = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
