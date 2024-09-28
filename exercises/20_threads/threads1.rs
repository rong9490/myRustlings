// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.

// Thread 现成实例, 与其他语言没有太大区别;

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
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("Thread {i} done");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }

    let mut results: Vec<u128> = Vec::new();
    for handle in handles {
        // TODO: Collect the results of all threads into the `results` vector.
        // Use the `JoinHandle` struct which is returned by `thread::spawn`.

        // thread.join() 方法, 作用是, 执行时长毫秒
        // 返回的是一个Result结果, 所以需要使用match处理!
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
