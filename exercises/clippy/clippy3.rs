// clippy3.rs
//
// 这里有几个更简单的 Clippy 修复点，让你见识一下它的用处。
//
// 执行 `rustlings hint clippy3` 或使用 `hint` watch 子命令来获取提示。

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 来把这两个值交换一下！
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
