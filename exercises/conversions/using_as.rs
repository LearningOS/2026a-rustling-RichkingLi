// using_as.rs
//
// Rust 里的类型转换通过 `as` 运算符完成。注意 `as` 不只用于类型转换，
// 它也用来给导入（import）起别名。
//
// 目标是让除法运算能编译通过，并返回正确的类型。
//
// 执行 `rustlings hint using_as` 或 `hint` watch 子命令查看提示。

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    total / values.len() as f64
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
