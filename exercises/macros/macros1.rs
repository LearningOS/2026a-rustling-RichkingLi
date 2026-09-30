// macros1.rs
//
// 执行 `rustlings hint macros1` 或使用 `hint` watch 子命令来获取提示。


macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
