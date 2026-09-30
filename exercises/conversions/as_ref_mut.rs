// as_ref_mut.rs
//
// AsRef 与 AsMut 用于廉价的“引用到引用”转换。分别见
// https://doc.rust-lang.org/std/convert/trait.AsRef.html 与
// https://doc.rust-lang.org/std/convert/trait.AsMut.html。
//
// 执行 `rustlings hint as_ref_mut` 或 `hint` watch 子命令查看提示。

// 取得给定参数的字节数（而非字符数）。
// TODO: 在 trait bound 中适当加上 AsRef trait。
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// 取得给定参数的字符数（而非字节数）。
// TODO: 在 trait bound 中适当加上 AsRef trait。
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// 用 as_mut() 对一个数字平方。
// TODO: 加上恰当的 trait bound。
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    // TODO: 实现函数体。
    let val = arg.as_mut();
    *val *= *val;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
