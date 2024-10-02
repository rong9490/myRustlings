// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

struct JobStatus {
    jobs_done: u32,
}

// atomic lock

fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state.

    // 跨线程间, 共享数据变量 原子"共享上下文"
    // 核心是理解, 原子锁 与 线程共享状态规则
    // Arc::new + Mutex::new
    let status: Arc<Mutex<JobStatus>> = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for _ in 0..15 {
        // 从原子上下文中取出共享的计数变量, 加一, 加上了锁; 防止抢占, 冲突
        let status_shared = Arc::clone(&status);

        // 产生线程
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(500)); // 线程休眠 --> 线程并发操作共享变量, 会存在抢占与锁的问题

            // TODO: You must take an action before you update a shared value.
            // 最简单的做法是, 加上一把锁, 每个操作都串行去做!
            status_shared.lock().unwrap().jobs_done += 1;
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Print the value of `JobStatus.jobs_done`.
    println!("Jobs done: {}", status.lock().unwrap().jobs_done);
}
