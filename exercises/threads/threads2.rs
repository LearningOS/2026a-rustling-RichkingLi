// threads2.rs
//
// 在上一题的基础上，我们同样要等所有线程完成各自的工作，但这次启动的线程还需要
// 负责更新一个共享值：JobStatus.jobs_completed。
//
// 执行 `rustlings hint threads2` 或使用 `hint` watch 子命令来获取提示。

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 在更新一个共享值之前，你必须先做一个动作：加锁
            status_shared.lock().unwrap().jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }
}
