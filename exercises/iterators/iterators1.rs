// iterators1.rs
//
// 在对集合中的元素执行操作时，迭代器（iterator）是必不可少的。本模块帮你熟悉迭代器的
// 使用结构，以及如何遍历一个可迭代集合里的元素。
//
// 通过填充 `???` 让我能编译通过。
//
// 执行 `rustlings hint iterators1` 或使用 `hint` watch 子命令来获取提示。

fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter();   // TODO: 第 1 步

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));     // TODO: 第 2 步
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));     // TODO: 第 3 步
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), None);     // TODO: 第 4 步
}
