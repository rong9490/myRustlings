pub fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec; // 移动语义(move semantics), 同时修改为可变
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
    fn move_semantics1() {
        let vec0: Vec<i32> = vec![22, 44, 66];
        let vec1: Vec<i32> = fill_vec(vec0); // vec0被move到函数里(作用域已经失效了!) --> vec1
        assert_eq!(vec1, vec![22, 44, 66, 88]); // 比较长度和各元素, 不是地址
    }
}
