// options3.rs
//
// 运行 `rustlings hint options3` 或使用 `hint` watch 子命令可以查看提示。


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // 在不删除这一行的前提下修复它。
}
