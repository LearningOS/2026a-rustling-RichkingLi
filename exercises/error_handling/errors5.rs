// errors5.rs
//
// 这个程序使用了 errors4 代码的一个改动版。
//
// 这个练习会用到一些课程后面才会讲到的概念，比如 `Box` 和 `From`
// trait。现在不必深究它们，但感兴趣的话可以提前读一读。眼下你可以把
// `Box<dyn ???>` 类型理解为"我想要任何能做 ??? 这件事的东西"的类型——
// 以 Rust 一贯的运行时安全标准来看，这在你眼里应该算相当宽松了！
//
// 简单说，Box 的这种用法适用于：你想拥有一个值，并且只关心它是实现了某
// 个特定 trait 的类型。为此，把 Box 声明为 Box<dyn Trait>，其中 Trait
// 就是编译器在该上下文中对每个用到的值去查找的 trait。在本练习里，这个
// 上下文就是 Result 里可能被返回的那些错误。
//
// 我们可以用什么来同时描述这两种错误？换句话说，有没有一个两种错误都实
// 现了的 trait？
//
// 执行 `rustlings hint errors5` 或使用 `hint` watch 子命令来获取提示。


use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO：修改 `main()` 的返回类型，让这个程序能编译通过。
fn main() -> Result<(), Box<dyn error::Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// 不要修改这一行以下的任何内容。

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// 这是为了让 `CreationError` 能够实现 `error::Error` 所必需的。
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
