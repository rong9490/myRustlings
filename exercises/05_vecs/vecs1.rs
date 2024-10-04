fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a: [i32; 4] = [10, 20, 30, 40]; // Array 数组, 可以在栈上, 也可以放到堆上

    // TODO: Create a vector called `v` which contains the exact same elements as in the array `a`.
    // Use the vector macro.
    let v: Vec<i32> = vec![10, 20, 30, 40]; // 容器, 存在堆上

    (a, v) // 通过元组一同返回
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        // a是数组, v是容器
        // 如何比较两者相等 a == *v
        let (a, v) = array_and_vec();
        assert_eq!(a, *v); // TODO 理解容器解引用后*v是什么类型
        assert_eq!(a, v[..]); // TODO 理解容器切片是什么类型
    }
}
