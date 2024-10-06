// You can use the `use` keyword to bring module paths from modules from
// anywhere and especially from the standard library into your scope.

// your scope. Bonus style points if you can do it with one line!

use std::time::{SystemTime, UNIX_EPOCH}; // 引入系统内置模块

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
