//! 键盘输入模拟模块
//! 
//! 使用 enigo 库模拟键盘输入

use anyhow::Result;
use enigo::{Enigo, KeyboardControllable};

/// 模拟键盘输入文本
pub fn simulate_key_input(text: &str) -> Result<()> {
    let mut enigo = Enigo::new();
    enigo.key_sequence(text);
    Ok(())
}
