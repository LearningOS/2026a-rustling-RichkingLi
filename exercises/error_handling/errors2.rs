// errors2.rs
//
// 假设我们在写一个游戏，玩家可以用代币购买物品。每件物品 5 个代币，而且
// 每次购买还有 1 个代币的手续费。玩家会输入想购买的数量，然后
// `total_cost` 函数会计算总共要花多少代币。不过，既然数量是玩家敲进来
// 的，我们拿到的就是一个字符串——而且他们什么都有可能输，可不一定只有
// 数字！
//
// 眼下这个函数完全没有处理错误情况（其实成功情况也处理得不对）。我们要
// 做的是：如果对一个不是数字的字符串调用 `parse` 函数，该函数会返回一个
// `ParseIntError`；遇到这种情况，我们要立即把这个错误从我们的函数里返回
// 出去，而不是接着去做乘法和加法。
//
// 至少有两种写法都正确——但其中一种要短得多！
//
// 执行 `rustlings hint errors2` 或使用 `hint` watch 子命令来获取提示。


use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
