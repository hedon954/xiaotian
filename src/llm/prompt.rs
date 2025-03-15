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
            .with_instruction("以下是项目的最新进展，请你根据内容合并同类项，形成一份简报。")
            .with_requirements(
                "简报中至少包含以下内容:\n1) 新增功能\n2) 主要改进\n3) 修复问题\n4) 参考链接",
            )
            .with_content(updates_content)
            .with_raw("报告请使用中文，并以优雅的 markdown 格式呈现。")
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

    #[test]
    fn test_build_report_summary() {
        let updates = "PR #123: 添加新功能\nIssue #456: 修复bug";
        let prompt = PromptBuilder::build_report_summary(updates);

        assert!(prompt.contains("以下是项目的最新进展"));
        assert!(prompt.contains("PR #123: 添加新功能"));
        assert!(prompt.contains("Issue #456: 修复bug"));
        assert!(prompt.contains("1) 新增功能"));
        assert!(prompt.contains("使用中文"));
    }
}
