// iterators4.rs
//
// 执行 `rustlings hint iterators4` 或使用 `hint` watch 子命令来获取提示。

pub fn factorial(num: u64) -> u64 {
    // 补全这个函数，返回 num 的阶乘
    // 不要使用：
    // - return
    // - 命令式风格循环（for、while）
    // - 额外变量
    // 想要额外挑战的话，也不要使用：
    // - 递归
    // 执行 `rustlings hint iterators4` 获取提示。
    (1..=num).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
