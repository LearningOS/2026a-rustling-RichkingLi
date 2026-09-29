// options2.rs
//
// 运行 `rustlings hint options2` 或使用 `hint` watch 子命令可以查看提示。


#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: 把这里改写成 if let 语句，匹配值是 "Some" 的情况
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: 把这里改写成 while let 语句 —— 注意 vector.pop 的返回值
        // 还会再多包一层 Option<T>。Option<T> 是可以堆叠嵌套的，
        // while let 和 if let 都能一次解开多层。
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
