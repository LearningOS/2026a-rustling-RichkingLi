// lifetimes1.rs
//
// Rust 编译器需要知道如何检查传入的引用是否有效，这样才能在“引用在
// 被使用之前就出了作用域”的风险出现时提醒程序员。记住，引用是借用，
// 并不拥有它指向的数据。万一它的所有者先出了作用域呢？
//
// 执行 `rustlings hint lifetimes1` 或使用 `hint` watch 子命令来获取提示。


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
