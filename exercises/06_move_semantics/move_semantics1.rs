// TODO: Fix the compiler error in this function.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    // 重新声明了一次, 但是并没有深拷贝! 依然是同一片内存地址s
    // 重新声明, 使其可以被修改mut
    let mut vec = vec;

    vec.push(88);

    vec
}

// move semantics = 移动语义

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        // vec0的作用域已经结束了, 被move到函数里了, 没有返回就自动失效;
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
