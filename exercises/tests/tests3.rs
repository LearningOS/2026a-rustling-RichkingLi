// tests3.rs
//
// 这个测试并没有在测试我们的函数 —— 改写它，让测试能通过。然后再写一个测试，
// 检验当我们调用 `is_even(5)` 时是否能得到预期的结果。
//
// 执行 `rustlings hint tests3` 或使用 `hint` watch 子命令来获取提示。


pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(2));
    }

    #[test]
    fn is_false_when_odd() {
        assert!(is_even(2));
    }
}
