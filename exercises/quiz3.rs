// quiz3.rs
//
// 本测验考察：
// - 泛型（Generics）
// - trait
//
// 一所想象中的魔法学校用 Rust 写了一套新的成绩单生成系统！目前这套系统
// 只支持生成“成绩用数字表示”的成绩单（比如 1.0 -> 5.5）。但学校也会发
// “字母成绩”（A+ 到 F-），而且需要能打印这两种类型的成绩单！
//
// 在 `ReportCard` 结构体和它的 impl 块里做必要的代码改动，让系统也能支持
// 字母成绩。把第二个测试里的成绩改成 "A+"，以证明你的改动确实能支持
// 字母成绩。
//
// 执行 `rustlings hint quiz3` 或使用 `hint` watch 子命令来获取提示。


use std::fmt::Display;

pub struct ReportCard<T: Display> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T: Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO：做完本题后，记得把这里的成绩改成 "A+"。
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
