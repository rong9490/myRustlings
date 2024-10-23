// In this exercise, we are given a `Vec` of `u32` called `numbers` with values
// ranging from 0 to 99. We would like to use this set of numbers within 8
// different threads simultaneously. Each thread is going to get the sum of
// every eighth value with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, …
// The second thread (offset 1), will sum 1, 9, 17, …
// The third thread (offset 2), will sum 2, 10, 18, …
// …
// The eighth thread (offset 7), will sum 7, 15, 23, …
//
// Each thread should own a reference-counting pointer to the vector of
// numbers. But `Rc` isn't thread-safe. Therefore, we need to use `Arc`.
//
// Don't get distracted by how threads are spawned and joined. We will practice
// that later in the exercises about threads.

// Don't change the lines below.
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

// 借鉴c++的特性:  智能指针, 结构体, 引用信息, 视为普通变量,
// 额外信息, 通用的模式, 场景, 

fn main() {
    // 生成0到100的容器
    let numbers: Vec<_> = (0..100u32).collect();

    // 解决方法: clone耗内存; 引用计数; 原子的, 可以多线程使用, 自动加减;
    // 包装一层, Arc指针 = Atomic reference count; 多线程之间使用,
    // 当我们clone时, 它自动加一
    // shared_numbers使用时, 会自动解引用, 方便获取值(Deref)
    // TODO: Define `shared_numbers` by using `Arc`.
    let shared_numbers = Arc::new(numbers);
    
    // Rc是线程内的计数; Arc是线程间的计数;

    let mut join_handles = Vec::new();

    // 按余数拆分给8个线程分别处理: numbers共享!
    for offset in 0..8 {
        // TODO: Define `child_numbers` using `shared_numbers`.
        // 这里两种写法一样!! 用Arc::clone 更显式明确
        // child_numbers所有权移动到子线程中(move), 但是不影响
        let child_numbers = Arc::clone(&shared_numbers);
        // let child_numbers = shared_numbers.clone();

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}
