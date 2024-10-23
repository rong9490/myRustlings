pub fn fill_vec(/* 拥有所有权可以修改 */mut vec: Vec<i32>) -> Vec<i32> {
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
    fn move_semantics3() {
        let vec0: Vec<i32> = vec![22, 44, 66];
        let vec1: Vec<i32> = fill_vec(vec0); // vec0已经移动失效了!!
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
