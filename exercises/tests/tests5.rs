// tests5.rs
//
// 安全（safe）代码由编译器保证内存安全，而不安全（unsafe）代码则把这一保证交给你自己来负责。
// 只要不违反任何契约（contract），你就可以用 `unsafe` 关键字来绕过编译器的某些检查。
//
// 由于契约的内容无法只用单个关键字来表达，你必须在本项文档注释的 `# Safety`
// 小节里手动说明它。
//
// 当 `unsafe` 标注在一个由花括号包裹的代码块上时，它表示遵守了某项契约，例如某个
// 指针参数的有效性、某段内存地址的所有权。但和上面那段文字一样，你仍然需要在该代码块的
// 注释中说明契约是如何被遵守的。
//
// 注意：所有注释都只是为了提升代码的可读性与可维护性，而 Rust 编译器把代码健全性（soundness）
// 的信任交给了你自己！如果你无法证明自己代码的内存安全性与健全性，那就退一步，改用安全代码！
//
// 执行 `rustlings hint tests5` 或使用 `hint` watch 子命令来获取提示。


/// # Safety
///
/// 参数 `address` 必须包含一个指向有效 `u32` 值的可变引用。
unsafe fn modify_by_address(address: usize) {
    // TODO: 在下方代码块中填写你的安全说明，使其与代码行为以及本函数的契约相符。
    // 你可以参考下方测试的注释格式。
    unsafe {
        //todo!("Your code goes here")
        let ptr = address as *mut u32;   // 整数地址 → 裸指针
        *ptr = 0xAABBCCDD;               // 解引用写入
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: 该地址保证有效，且包含一个指向局部 `u32` 变量的独占引用。
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
