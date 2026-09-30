// box1.rs
//
// 在编译期，Rust 需要知道一个类型占用多少空间。对于递归类型（一个值内部可以包含另一个
// 相同类型的值）来说，这就成了问题。为了绕开这个限制，我们可以使用 `Box` —— 一种把数据
// 存到堆上的智能指针，它也能让我们包裹递归类型。
//
// 本练习要实现的递归类型是 `cons list`（构造列表）——函数式编程语言里常见的一种数据结构。
// cons list 中的每一项都包含两个元素：当前项的值，以及下一项。最后一项是一个叫 `Nil` 的值。
//
// 第 1 步：在枚举定义里使用 `Box`，让代码能编译
// 第 2 步：用替换 `todo!()` 的方式，创建空和非空的 cons list
//
// 注意：不要修改测试
//
// 执行 `rustlings hint box1` 或使用 `hint` watch 子命令来获取提示。

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    let b = Box::new(List::Nil);
    List::Cons(5, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
