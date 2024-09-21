fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x; // 可变引用y
        y.push(42);
        // y.free(); 相当于隐式调用了

        let z = &mut x; // 可变引用z
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
