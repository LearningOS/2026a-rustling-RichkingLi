// generics2.rs
//
// 这个功能强大的包装器提供了存储正整数值的能力。用泛型重写它，让它能
// 包装**任意**类型。
//
// 执行 `rustlings hint generics2` 或使用 `hint` watch 子命令来获取提示。


struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
