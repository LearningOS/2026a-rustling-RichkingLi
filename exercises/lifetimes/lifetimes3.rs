// lifetimes3.rs
//
// 当结构体持有引用时，同样也需要生命周期标注。
//
// 执行 `rustlings hint lifetimes3` 或使用 `hint` watch 子命令来获取提示。

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
