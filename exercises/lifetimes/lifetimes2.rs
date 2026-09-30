// lifetimes2.rs
//
// 既然编译器只是在验证传给那些被标注参数和返回类型的引用，那我们到底
// 需要改动什么？
//
// 执行 `rustlings hint lifetimes2` 或使用 `hint` watch 子命令来获取提示。


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is '{}'", result);
}
