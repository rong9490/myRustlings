fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // 为什么切片不能直接切a, 而要切a的引用: 因为a是指针, 实际需要切内存空间
        let nice_slice: &[i32] = &a[1..4];

        assert_eq!([2, 3, 4], nice_slice);
    }
}
