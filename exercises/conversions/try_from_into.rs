// try_from_into.rs
//
// TryFrom 是一种简单且安全的类型转换，在某种情况下可能以受控的方式失败。
// 本质上它与 From 相同，主要区别是它返回的是 Result 类型，而非目标类型本身。
// 更多内容见 https://doc.rust-lang.org/std/convert/trait.TryFrom.html
//
// 执行 `rustlings hint try_from_into` 或 `hint` watch 子命令查看提示。

use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// 我们将用这个错误类型来做这些 `TryFrom` 转换。
#[derive(Debug, PartialEq)]
enum IntoColorError {
    // 切片长度不正确
    BadLen,
    // 整数转换错误
    IntConversion,
}

// 你的任务是补全这个实现并返回 Ok 包裹的 Color 类型结果。你需要为
// “三个整数组成的元组”“三个整数组成的数组”以及“整数切片”分别实现。
//
// 注意：元组和数组的实现会在编译期被检查，但切片实现需要自行检查切片长度！
// 另外注意，合法的 RGB 颜色值必须是 0..=255 范围内的整数。

// 内部辅助：把三个 i16 分量逐个转换成 u8（0..=255 检查由 u8::try_from 完成），
// 任一分量越界（含负数与 >255）就返回 IntConversion。
fn color_try_from_parts(r: i16, g: i16, b: i16) -> Result<Color, IntoColorError> {
    let red = u8::try_from(r).map_err(|_| IntoColorError::IntConversion)?;
    let green = u8::try_from(g).map_err(|_| IntoColorError::IntConversion)?;
    let blue = u8::try_from(b).map_err(|_| IntoColorError::IntConversion)?;
    Ok(Color { red, green, blue })
}

// 元组实现
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let (r, g, b) = tuple;
        color_try_from_parts(r, g, b)
    }
}

// 数组实现
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        color_try_from_parts(arr[0], arr[1], arr[2])
    }
}

// 切片实现
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen); // 运行时检查长度（编译期无法知道切片长度）
        }
        color_try_from_parts(slice[0], slice[1], slice[2])
    }
}

fn main() {
    // 使用 `try_from` 函数
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    // 既然为 Color 实现了 TryFrom，我们应当也能用 TryInto
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    // 对切片要用 `try_from` 函数
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);
    // 或在括号里取切片并用 TryInto
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert_eq!(
            Color::try_from((256, 1000, 10000)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_out_of_range_negative() {
        assert_eq!(
            Color::try_from((-1, -10, -256)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_sum() {
        assert_eq!(
            Color::try_from((-1, 255, 255)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
}
