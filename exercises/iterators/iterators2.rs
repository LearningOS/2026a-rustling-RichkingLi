// iterators2.rs
//
// 在这个练习里，你会学到迭代器能提供的一些独特优势。按步骤完成练习。
//
// 执行 `rustlings hint iterators2` 或使用 `hint` watch 子命令来获取提示。

// 第 1 步。
// 补全 `capitalize_first` 函数。
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(c).collect(),
    }
}

// 第 2 步。
// 把 `capitalize_first` 函数应用到字符串切片的切片上。
// 返回一个字符串向量。
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| capitalize_first(word)).collect()
}

// 第 3 步。
// 再次把 `capitalize_first` 函数应用到字符串切片的切片上。
// 返回一个单独的字符串。
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|word| capitalize_first(word)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
