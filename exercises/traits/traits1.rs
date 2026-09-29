// traits1.rs
//
// 是时候实现一些 trait 了！你的任务是为 `String` 类型实现 `AppendBar`
// trait。该 trait 只有一个函数，作用是给任何实现了它的对象追加 "Bar"。
//
// 执行 `rustlings hint traits1` 或使用 `hint` watch 子命令来获取提示。


trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO：为 `String` 类型实现 `AppendBar`。
    fn append_bar(self) -> Self {
        let mut s = self;
        s.push_str("Bar");
        s
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
