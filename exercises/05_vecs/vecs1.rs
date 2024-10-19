pub fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a: [i32; 4] = [10, 20, 30, 40]; // Array 数组, 可以在栈上, 也可以放到堆上
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
        let (a, mut v) = array_and_vec();

        assert_eq!(a.len(), 4); // 数组长度为4字节
        assert_eq!(std::mem::size_of_val(&a[0]), 4); // 数组元素i32大小为4字节
        assert_eq!(
            std::mem::size_of_val(&a),
            a.len() * std::mem::size_of_val(&a[0])
        ); // 数组大小为4*4=16字节

        assert_eq!(v.len(), 4); // 容器长度为4
        assert_eq!(v.capacity(), 4);
        assert_eq!(std::mem::size_of_val(&v[0]), 4);
        assert_eq!(std::mem::size_of_val(&v[0]) * v.len(), 4 * 4);

        assert_ne!(
            std::mem::size_of_val(&v[0]) * v.len(),
            std::mem::size_of_val(&v) + 8
        );
        assert_eq!(std::mem::size_of_val(&v), 24);
        assert_eq!(v.is_empty(), false);

        // 两端都是"切片", 数组可以隐式转换为切片
        // 容器解引用或者切片后, 都是切片!!
        // Rust的切片仅比较: 长度和每个元素, 不看内存地址!! 所以比较结果为True
        assert_eq!(a, *v);
        assert_eq!(a, v[..]);

        v.push(50);
        v.push(60);
        v.push(70);
        assert_eq!(v.len(), 7);
        assert_eq!(std::mem::size_of_val(&v[0]) * v.len(), 4 * 7);
        assert_eq!(std::mem::size_of_val(&v), 24); // 容器为智能指针, 固定大小为24字节, 不随内容而改变
    }
}
