// You can use the `use` keyword to bring module paths from modules from
// anywhere and especially from the standard library into your scope.

use std::time::{Duration, SystemTime, SystemTimeError, UNIX_EPOCH}; // 引入系统内置模块

fn main() {
    let time: Result<Duration, SystemTimeError> = SystemTime::now().duration_since(UNIX_EPOCH);
    match time {
        Ok(n) => {
            let seconds: u64 = n.as_secs();
            println!("1970-01-01 00:00:00 UTC was {} seconds ago!", seconds);
        }
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
