// cow1.rs
//
// 这个练习探索 Cow，即 Clone-On-Write（写时克隆）类型。Cow 是一个写时克隆的智能指针。
// 它能包裹借来的数据并提供不可变访问；当需要做修改或需要所有权时，才惰性地把数据克隆出来。
// 这个类型通过 Borrow trait 设计成能配合通用的借来数据使用。
//
// 本练习意在让你了解：把数据传给 Cow 时会发生什么。在 TODO 标记处检查
// Cow::Owned(_) 和 Cow::Borrowed(_)，从而修复单元测试。
//
// 执行 `rustlings hint cow1` 或使用 `hint` watch 子命令来获取提示。

use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // 如果尚未拥有数据，则克隆成一个向量。
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // 发生克隆，因为 `input` 需要被修改。
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // 不发生克隆，因为 `input` 不需要被修改。
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Borrowed(_) => Ok(()),
            _ => Err("Expected borrowed value"),
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // 我们也可以不加 `&` 直接把 `slice` 传进去，这样 Cow 直接拥有它。这种情况下
        // 不发生修改，因而也不克隆；但结果仍是 owned，因为它从未被借出或修改过。
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // 当然，如果真发生了修改，也是同样的情况。此时对 `to_mut()` 的调用返回的
        // 还是和之前相同的数据的引用。
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }
}
