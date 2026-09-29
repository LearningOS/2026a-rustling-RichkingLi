// traits2.rs
//
// 你的任务是为一个字符串向量（vector）实现 `AppendBar` trait。实现时先想
// 一下：给"一个字符串向量"追加 "Bar"，到底意味着什么。
//
// 这次没有样板代码了，你自己能搞定！
//
// 执行 `rustlings hint traits2` 或使用 `hint` watch 子命令来获取提示。


trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO：为字符串向量实现 `AppendBar` trait。

impl AppendBar for Vec<String> {
    // TODO：为 `String` 类型实现 `AppendBar`。
    fn append_bar(self) -> Self {
        let mut s = self;
        s.push(String::from("Bar"));
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
