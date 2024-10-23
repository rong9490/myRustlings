fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn move_semantics4() {
        let mut x: Vec<i32> = Vec::new();

        let y: &mut Vec<i32> = &mut x; // x 的一个可变引用 y
        y.push(42); // y.free(); 隐式释放内存了

        let z: &mut Vec<i32> = &mut x; // x 的另一个可变引用 z
        z.push(13); // z.free(); 隐式释放内存了

        // HACK 只要不是同时存在两个可变引用就行
        assert_eq!(x, [42, 13]);
    }
}
