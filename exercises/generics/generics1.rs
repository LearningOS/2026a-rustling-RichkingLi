// generics1.rs
//
// 这个购物清单程序编译不过！用你的泛型知识修好它。
//
// 执行 `rustlings hint generics1` 或使用 `hint` watch 子命令来获取提示。


fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
