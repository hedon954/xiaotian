pub mod commands;
pub mod error;
// pub mod repl;  // 删除这行，避免模块名称冲突

// 将原来的 pub use repl::Repl; 替换为以下导出方式
mod implementation; // 引入包含 Repl 实现的模块
pub use implementation::Repl; // 从该模块导出 Repl 结构体
