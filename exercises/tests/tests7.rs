// tests7.rs
//
// 在构建包的时候，有些依赖既无法在 `Cargo.toml` 中导入，也无法直接链接；有些预处理步骤
// 则因项目而异，从代码生成到设置项目专属配置都有。
//
// Cargo 并不打算取代其他构建工具，但它确实通过名为 `build.rs` 的自定义构建脚本与它们集成。
// 这个文件通常放在项目根目录，而本练习中则放在与本文件相同的目录里。
//
// 它可以用于：
//
// - 构建捆绑的 C 语言库。
// - 在宿主机系统上查找 C 语言库。
// - 根据规范生成 Rust 模块。
// - 执行 crate 所需的任何平台相关配置。
//
// 在设置配置时，我们可以在构建脚本里用 `println!` 告知 Cargo 遵循某些指令。通用格式为：
//
//     println!("cargo:{}", your_command_in_string);
//
// 更多关于构建脚本的信息，请参阅官方的 Cargo 文档：
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
// 在本练习中，我们要查找一个环境变量，并期望它落在某个范围之内。你可以查看测试用例来了解细节。
//
// 你不应修改本文件。请修改同一目录下的 `build.rs` 来通过本练习。
//
// 执行 `rustlings hint tests7` 或使用 `hint` watch 子命令来获取提示。

// I AM NOT DONE

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let s = std::env::var("TEST_FOO").unwrap();
        let e: u64 = s.parse().unwrap();
        assert!(timestamp >= e && timestamp < e + 10);
    }
}
