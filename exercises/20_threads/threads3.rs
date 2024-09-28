use std::{sync::mpsc, thread, time::Duration};

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

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) {
    // 使用 clone() 克隆 tx 发送者,这样可以在两个线程中使用
    // 理解为什么需要 clone ??
    let tx1 = tx.clone();
    

    // move所有权进入
    thread::spawn(move || {
        for val in q.first_half {
            println!("发送 {val:?}");
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    thread::spawn(move || {
        for val in q.second_half {
            println!("发送 {val:?}");
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });
}

fn main() {
    // 你可以在这里进行可选的实验
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threads3() {
        let (tx, rx) = mpsc::channel();
        let queue = Queue::new();

        send_tx(queue, tx);

        let mut received = Vec::with_capacity(10);
        for value in rx {
            received.push(value);
        }

        received.sort();
        assert_eq!(received, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
