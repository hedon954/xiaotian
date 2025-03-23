//! 提示词构建工具

/// 提示词构建器，帮助构建结构化的提示词
#[derive(Debug, Clone, Default)]
pub struct PromptBuilder {
    parts: Vec<String>,
}

impl PromptBuilder {
    /// 创建一个新的提示词构建器
    pub fn new() -> Self {
        Self { parts: Vec::new() }
    }

    /// 添加指令部分
    pub fn with_instruction(mut self, instruction: impl AsRef<str>) -> Self {
        self.parts
            .push(format!("# 指令\n{}\n", instruction.as_ref()));
        self
    }

    /// 添加上下文部分
    pub fn with_context(mut self, context: impl AsRef<str>) -> Self {
        self.parts.push(format!("# 上下文\n{}\n", context.as_ref()));
        self
    }

    /// 添加内容部分
    pub fn with_content(mut self, content: impl AsRef<str>) -> Self {
        self.parts.push(format!("# 内容\n{}\n", content.as_ref()));
        self
    }

    /// 添加要求部分
    pub fn with_requirements(mut self, requirements: impl AsRef<str>) -> Self {
        self.parts
            .push(format!("# 要求\n{}\n", requirements.as_ref()));
        self
    }

    /// 添加示例部分
    pub fn with_examples(mut self, examples: impl AsRef<str>) -> Self {
        self.parts.push(format!("# 示例\n{}\n", examples.as_ref()));
        self
    }

    /// 添加自定义部分
    pub fn with_custom(mut self, title: impl AsRef<str>, content: impl AsRef<str>) -> Self {
        self.parts
            .push(format!("# {}\n{}\n", title.as_ref(), content.as_ref()));
        self
    }

    /// 添加原始文本，不添加标题
    pub fn with_raw(mut self, raw: impl AsRef<str>) -> Self {
        self.parts.push(raw.as_ref().to_string());
        self
    }

    /// 构建最终的提示词文本
    pub fn build(&self) -> String {
        self.parts.join("\n")
    }

    /// 构建用于报告总结的提示词
    pub fn build_report_summary(updates_content: &str) -> String {
        Self::new()
            .with_instruction("根据项目更新内容，生成一份结构化的中文周报，突出重要变更。")
            .with_requirements(
                r#"
# {项目名称} 周报
## 更新概览
- 更新数量统计
- 主要变更类型分布

## 核心更新
### 🌟 功能增强
- 新特性及影响
- 重要功能改进

### ⚡ 性能优化
- 性能提升要点
- 具体改进数据

### 🔧 重要修复
- 关键bug修复
- 影响范围说明

## 社区动态
- 重要提案进展
- 热点讨论焦点

## 展望
- 后续开发计划
- 待解决的问题

## 参考链接
[相关PR/Issue链接]"#,
            )
            .with_content(updates_content)
            .with_raw("使用markdown格式，保持结构清晰，突出实质性影响。省略无相关内容的章节。")
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_builder() {
        let prompt = PromptBuilder::new()
            .with_instruction("总结以下内容")
            .with_content("这是一段测试内容")
            .with_requirements("使用中文回答")
            .build();

        assert!(prompt.contains("# 指令"));
        assert!(prompt.contains("总结以下内容"));
        assert!(prompt.contains("# 内容"));
        assert!(prompt.contains("这是一段测试内容"));
        assert!(prompt.contains("# 要求"));
        assert!(prompt.contains("使用中文回答"));
    }
}
