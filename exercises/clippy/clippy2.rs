// clippy2.rs
// 
// 执行 `rustlings hint clippy2` 或使用 `hint` watch 子命令查看提示。


fn main() {
    let mut res = 42;
    let option = Some(12);
    //for x in option {
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
