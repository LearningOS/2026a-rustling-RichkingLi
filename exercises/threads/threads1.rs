// threads1.rs
//
// 这个程序会启动多个线程，每个线程至少运行 250ms，并且每个线程返回自己花了多少
// 时间完成。程序应当等所有启动的线程都跑完，并把它们的返回值收集到一个向量里。
//
// 执行 `rustlings hint threads1` 或使用 `hint` watch 子命令来获取提示。

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
