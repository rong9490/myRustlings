use std::{sync::mpsc, thread, time::Duration};
use std::sync::mpsc::Sender;
// sync::mpsc = multiple producer, single consumer 多生产者, 单消费者

#[derive(Debug)]
struct Queue {
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Self {
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

// HACK 这道题没看懂
// mpsc 通讯: 共享 与 消息传递
// mpsc 多个生产者, 一个消费者; 拿到队列的消息
// Send , Sync; T is Sync: &T is send;

// mpsc: Sender<u32> 这里参数传入是有所有权的, 但是有两次需要使用, 所以需要clone
// 是否支持clone: impl<T> Clone for Sender<T> { inner: self.inner.clone() }
fn send_tx(q: Queue, tx: mpsc::Sender<u32>) {
    // 使用 clone() 克隆 tx 发送者,这样可以在两个线程中使用
    // 因为spawn+move, 变量已经转移所有权了!
    let tx_cloned = tx.clone();

    // 线程1: 遍历first_half, 拿到第一个消息发送者, tx_cloned.send(val) 发送数字;
    thread::spawn(move || {
        for val in q.first_half {
            println!("发送 {val:?}");
            tx_cloned.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    // 线程2: 遍历second_half, 拿到第二个消息发送者, tx2.send(val) 发送数字;
    thread::spawn(move || {
        for val in q.second_half {
            println!("发送 {val:?}");
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });
}

fn main() {
    let queue = Queue::new(); // Struct实例化
    let (tx, rx) = mpsc::channel::<i32>(); // 实例化通讯频道
    let queue_first_length = queue.first_half.len();
    let queue_second_length = queue.second_half.len();

    println!("{:?}", queue);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threads3() {
        // tx为发送者, rx为接受者
        let (tx, rx) = mpsc::channel::<u32>(); // 实例化u32的消息通道
        let queue = Queue::new(); // 结构体实例化
    
        // 核心: 
        send_tx(queue, tx);

        let mut received = Vec::with_capacity(10);
        for value in rx {
            received.push(value);
        }

        received.sort();
        assert_eq!(received, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
