// from_into.rs
//
// From trait 用于“值到值”的转换。如果某个类型正确地实现了 From，
// 那么与之反向的 Into trait 也应能正常工作。更多内容见
// https://doc.rust-lang.org/std/convert/trait.From.html
//
// 执行 `rustlings hint from_into` 或 `hint` watch 子命令查看提示。

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// 你的任务是补全这个实现，使得 `let p = Person::from("Mark,20")` 这行能编译。
// 注意：你需要用类似 `"4".parse::<usize>()` 的方式把 age 部分解析成 `usize`，
// 并且要妥善处理解析结果。
//
// 步骤：
// 1. 如果传入字符串长度为 0，返回 Person 的默认值。
// 2. 按字符串中的逗号进行分割。
// 3. 从分割结果中取出第一个元素，用作 name。
// 4. 如果 name 为空，返回 Person 的默认值。
// 5. 从分割结果中取出另一个元素，将其解析为 `usize` 用作 age。
// 6. 解析 age 过程中若出错，返回 Person 的默认值；
//    否则返回一个用上述结果构造的 Person 实例。

impl Default for Person {
    fn default() -> Person {
        Person {
            name: "John".to_string(),
            age: 30,
        }
    }
}

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Person::default();
        }
        if parts[0].is_empty() || parts[1].is_empty() {
            return Person::default();
        }
        let name = parts[0].to_string();
        let age = match parts[1].parse::<usize>() {
            Ok(age) => age,
            Err(_) => return Person::default(),
        };
        Person { name, age }
    }
}

fn main() {
    // 使用 `from` 函数
    let p1 = Person::from("Mark,20");
    // 由于为 Person 实现了 From，我们也可以直接用 Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // 测试默认 Person 是 30 岁的 John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // 测试传入坏字符串时会返回 John
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // 测试 "Mark,20" 能正常工作
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // 测试 "Mark,twenty" 因 age 解析失败而返回默认 Person
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
