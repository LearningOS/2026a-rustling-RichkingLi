// errors1.rs
//
// 这个函数在你传入空字符串时，拒绝生成要打印在名牌上的文本。如果能说明
// 问题出在哪里，而不是只在部分情况下返回 `None`，体验会更好。好在 Rust
// 有一个与 `Result` 类似的构造，可以用来表达错误条件。我们用上它吧！
//
// 执行 `rustlings hint errors1` 或使用 `hint` watch 子命令来获取提示。


pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // 空名字是不允许的。
        Err("`name` was empty; it must be nonempty.".to_string())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // 不要修改这一行
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
