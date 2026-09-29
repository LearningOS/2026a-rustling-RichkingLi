// traits5.rs
//
// 你的任务是替换 '??' 部分，让代码能编译通过。
//
// 除了标记的那一行，不要改动其他任何行。
//
// 执行 `rustlings hint traits5` 或使用 `hint` watch 子命令来获取提示。


pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// 你只能修改下一行
fn some_func(item: impl SomeTrait + OtherTrait) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    some_func(SomeStruct {});
    some_func(OtherStruct {});
}
