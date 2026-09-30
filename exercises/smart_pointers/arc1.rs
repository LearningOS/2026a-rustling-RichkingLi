// arc1.rs
//
// 在这个练习里，我们拿到一个名为 "numbers" 的 u32 向量，值是 0 到 99 ——
// [ 0, 1, 2, ..., 98, 99 ]。我们想在 8 个不同的线程里同时使用这组数字。每个线程
// 会按偏移量，求"每隔 8 个取值"之和。
//
// 第一个线程（偏移 0）求 0, 8, 16, ...
// 第二个线程（偏移 1）求 1, 9, 17, ...
// 第三个线程（偏移 2）求 2, 10, 18, ...
// ...
// 第八个线程（偏移 7）求 7, 15, 23, ...
//
// 因为我们用了线程，数据必须线程安全。因此这里用 Arc。我们需要在两处 TODO 各做一处改动。
//
// 让代码编译通过：在第一个 TODO 注释处给 `shared_numbers` 填一个值，并在第二个 TODO 注释处
// 给 `child_numbers` 创建一个初始绑定。尽量不要对 `numbers` 向量做任何拷贝！
//
// 执行 `rustlings hint arc1` 或使用 `hint` watch 子命令来获取提示。

#![forbid(unused_imports)] // 不要改动这一行（以及下一行）。
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
