// traits3.rs
//
// 你的任务是为两个结构体都实现 `Licensed` trait，并且不重复写相同的函数，
// 让它们返回一模一样的信息。
//
// 想想你能往 `Licensed` trait 里加点什么。
//
// 执行 `rustlings hint traits3` 或使用 `hint` watch 子命令来获取提示。


pub trait Licensed {
    fn licensing_info(&self) -> String {
        "Some information".to_string()
    }
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // 不要修改这一行
impl Licensed for OtherSoftware {} // 不要修改这一行

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
