// macros3.rs
//
// 让我编译通过，但不许把宏移出这个模块（module）！
//
// 执行 `rustlings hint macros3` 或使用 `hint` watch 子命令来获取提示。

#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
