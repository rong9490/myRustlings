fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers: (i32, i32, i32) = (1, 2, 3);

        assert_eq!(numbers.2, 3);

        // and assign it to a variable called `second`.
        // 元组也是index索引访问
        let second: i32 = numbers.1;

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}
