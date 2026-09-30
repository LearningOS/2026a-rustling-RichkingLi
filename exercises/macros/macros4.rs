// macros4.rs
//
// 执行 `rustlings hint macros4` 或使用 `hint` watch 子命令来获取提示。

#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
