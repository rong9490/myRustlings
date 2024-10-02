// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.

// Thread 现成封装好的实例, 与其他语言没有太大区别;

use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    let mut handles = Vec::new(); // 列表保存

    // 产生10个线程, 存入handles句柄
    // 创建方法: thread::spawn + 闭包(move操作), 使用到的所有变量的所有权, 都将移动到闭包内
    // spawn 产生返回用于操作的句柄, 实例
    for i in 0..10 {
        let handle = thread::spawn(move || {
            let start = Instant::now(); // 开始时刻
            thread::sleep(Duration::from_millis(250)); // 休眠250ms
            println!("Thread {i} done"); // 打印线程结束
            start.elapsed().as_millis() // 返回花费的毫秒数
        });
        handles.push(handle);
    }

    let mut results: Vec<u128> = Vec::new();
    for handle in handles {
        // TODO: Collect the results of all threads into the `results` vector.
        // Use the `JoinHandle` struct which is returned by `thread::spawn`.

        // thread.join() 方法, 作用是, 执行时长毫秒
        // 收集Result 返回的是一个Result结果, 所以需要使用match处理!
        // join 返回耗时情况, map
        let _ = handle.join().map(|mills| results.push(mills));
    }

    // 检查长度
    if results.len() != 10 {
        panic!("Oh no! Some thread isn't done yet!");
    }

    for (i, result) in results.into_iter().enumerate() {
        println!("Thread {i} took {result}ms");
    }
}
