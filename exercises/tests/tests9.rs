// tests9.rs
//
// Rust 非常擅长与 C/C++ 及其他静态编译语言共享 FFI 接口，甚至能在代码内部直接链接！
// 它借助 `extern` 代码块来实现，就像下面的代码一样。
//
// `extern` 关键字后面的短字符串，表示被导入的外部函数所遵循的 ABI（应用二进制接口）。
// 本练习使用 "Rust"，此外还有其它变体，例如标准 C ABI 用 "C"、Windows ABI 用 "stdcall"。
//
// 被导入的外部函数在 `extern` 代码块中声明，用分号（而非花括号）标记签名的结束。可以给这些
// 函数声明加上一些属性来修改链接行为，例如 `#[link_name = ".."]` 可修改实际符号名。
//
// 如果你要把自己的符号导出到链接环境，`extern` 关键字也可以标在某个函数定义之前，并附带
// 相同的 ABI 字符串说明。Rust 函数的默认 ABI 其实就是 "Rust"，所以如果你想链接纯 Rust 函数，
// 整个 extern 部分都可以省略。
//
// 与 C++ 类似，Rust 默认会对符号进行改名（mangle）。要抑制这一行为、让这些函数能通过名字
// 被寻址，可以加上 `#[no_mangle]` 属性。
//
// 在本练习中，你的任务是让测试用例能够调用模块 Foo 里的 `my_demo_function`。
// `my_demo_function_alias` 是 `my_demo_function` 的别名，因此测试用例里的这两行代码应当调用
// 同一个函数。
//
// 除了新增两行属性之外，你不应修改任何已有代码。


extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    // 没有 `extern` 等同于 `extern "Rust"`。
    #[no_mangle]
    fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // 默认情况下，被导入的外部函数是不安全的（UNSAFE），因为它们来自不被信任的其它语言。
        // 你可以把它们包装成安全的 Rust API 来减轻调用方的负担。
        //
        // SAFETY: 我们知道这些函数其实是一个安全 Rust 函数的别名。
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
