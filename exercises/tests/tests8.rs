// tests8.rs
//
// 本练习与上一个练习共用同一个 `build.rs`。你需要往 `build.rs` 里添加一些代码，
// 让本练习和上一个练习都能通过。
//
// 执行 `rustlings hint tests8` 或使用 `hint` watch 子命令来获取提示。


fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        #[cfg(feature = "pass")]
        return;

        panic!("no cfg set");
    }
}
