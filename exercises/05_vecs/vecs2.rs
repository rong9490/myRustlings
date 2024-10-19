/// 切片转容器Vec
pub fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    // 这里可以不写iter(), 因为input是slice切片, 默认就是迭代器(Rust允许省略)
    for element in input.iter() {
        output.push((*element) * 2); // 解引用, 拿出指针里的值计算
    }
    output
}

// Map迭代器遍历方式: map + 闭包 + collect
pub fn vec_map_example(input: &[i32]) -> Vec<i32> {
    input
        .iter()
        // map的闭包, 解引用取出值计算
        .map(|element| (*element) + 1)
        // collect收集回容器Vec(具体什么内省, turboFish自动推导)
        .collect::<Vec<i32>>()
}

pub fn vec_map(input: &[i32]) -> Vec<i32> {
    // iter()转迭代器; map+闭包; collect+turboFish收集回容器Vec
    input
        .iter()
        .map(|element| (*element) * 2)
        .collect::<Vec<i32>>()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input: [i32; 5] = [2, 4, 6, 8, 10];
        let ans: Vec<i32> = vec_loop(&input); // 数组即切片
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_vec_map_example() {
        let input: [i32; 3] = [1, 2, 3];
        let ans: Vec<i32> = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}
