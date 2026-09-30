// from_str.rs
//
// 这与 from_into.rs 类似，但这次我们要实现 `FromStr`，并在出错时返回错误
// 而不是回退到默认值。此外，实现 FromStr 之后，就可以对字符串调用 `parse`
// 方法来生成实现者类型的对象。更多内容见
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// 执行 `rustlings hint from_str` 或 `hint` watch 子命令查看提示。

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// 我们将用这个错误类型来实现 `FromStr`。
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // 输入字符串为空
    Empty,
    // 字段数量不正确
    BadLen,
    // name 字段为空
    NoName,
    // 包装了 parse::<usize>() 产生的错误
    ParseInt(ParseIntError),
}

// 步骤：
// 1. 如果传入字符串长度为 0，应返回错误
// 2. 按字符串中的逗号进行分割
// 3. 分割结果应当只返回 2 个元素，否则返回错误
// 4. 从分割结果取出第一个元素，用作 name
// 5. 从分割结果取出另一个元素，用类似 `"4".parse::<usize>()` 的方式
//    将其解析为 `usize` 作为 age
// 6. 在取出 name 和 age 的过程中若出错，应返回错误
// 若一切顺利，返回一个 Result 包裹的 Person 对象
//
// 补充：`Box<dyn Error>` 实现了 `From<&'_ str>`。这意味着如果你
// 想返回一个字符串错误信息，只需 `return Err("my error message".into())` 即可。

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        // 步骤 1：空串 → Empty
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }
        // 步骤 2、3：按逗号分割，必须恰好 2 段，否则 → BadLen
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }
        // 步骤 4：取 name，空 name → NoName（必须先于 age 解析检查，
        // 否则 ",1" 会因 age 解析成功而被误判为 Ok）
        let name = parts[0];
        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }
        // 步骤 5：解析 age，失败则把 ParseIntError 包进 ParseInt 变体
        let age = parts[1]
            .parse::<usize>()
            .map_err(ParsePersonError::ParseInt)?;
        // 步骤 6：一切顺利 → 返回 Ok 包裹的 Person
        Ok(Person {
            name: name.to_string(),
            age,
        })
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
