// macros2.rs
//
// 执行 `rustlings hint macros2` 或使用 `hint` watch 子命令来获取提示。



fn main() {
    my_macro!();
}

#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
