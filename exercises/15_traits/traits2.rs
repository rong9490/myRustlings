pub trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Vec<String> {
        // 容器Vec追加String
        self.push(String::from("Bar"));
        self
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo: Vec<String> = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.len(), 2);
        assert_eq!(std::mem::size_of_val(&foo), 24);

        let pop1: Option<String> = foo.pop();
        assert_eq!(foo.len(), 1);
        assert_eq!(pop1.is_some(), true);
        assert_eq!(pop1.unwrap(), "Bar");

        let pop2: Option<String> = foo.pop();
        assert_eq!(pop2.is_some(), true);
        assert_eq!(pop2.unwrap(), "Foo");
        assert_eq!(foo.len(), 0);
        assert_eq!(foo.is_empty(), true);
    }
}
