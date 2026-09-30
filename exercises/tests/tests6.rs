// tests6.rs
//
// 在这个例子里，我们浅尝一下 Rust 标准库中的不安全（unsafe）函数。填补所有的问号与 todo，
// 让测试通过。
//
// 执行 `rustlings hint tests6` 或使用 `hint` watch 子命令来获取提示。

struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// 指针 `ptr` 必须包含一个被拥有的 `Foo` 装箱（Box）。
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: 根据契约，`ptr` 包含一个被拥有的 `Foo` 装箱。我们只是据此重建这个 Box。
    let mut ret: Box<Foo> = unsafe { Box::from_raw(ptr) };
    ret.b = Some("hello".to_owned());
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize;
        // SAFETY: 我们传入了一个被拥有的 `Foo` 装箱。
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = &ret.a as *const u128 as usize;

        assert!(ptr_1 == ptr_2);
        assert!(ret.b == Some("hello".to_owned()));
    }
}
