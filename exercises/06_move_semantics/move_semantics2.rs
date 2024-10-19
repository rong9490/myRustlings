pub fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec: Vec<i32> = vec;
    vec.push(88);
    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics2() {
        let vec0: Vec<i32> = vec![22, 44, 66];
        let vec1: Vec<i32> = fill_vec(vec0.clone()); // 传入的是副本, 不影响原先的生命周期
        // clone开辟新堆内存, 两者的内存地址也不一样, 不会影响原始的vec0值
        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
