"""
模拟数据工具，用于在没有实际后端时提供测试数据
"""

MOCK_ORIGINAL_DATA = """
# golang/go 最新更新

## 提交记录

1. [cmd/compile: make more functions able to be inlined](https://github.com/golang/go/commit/abc123)
   - 作者：John Doe
   - 时间：2024-03-25
   - 描述：优化了编译器的内联功能，提高了代码执行效率

2. [net/http: improve server performance](https://github.com/golang/go/commit/def456)
   - 作者：Jane Smith
   - 时间：2024-03-24
   - 描述：提升了 HTTP 服务器的性能表现

## Pull Requests

1. [proposal: spec: allow type parameters on methods](https://github.com/golang/go/pull/789)
   - 状态：Open
   - 作者：Alice Johnson
   - 描述：提议在方法上支持类型参数

2. [cmd/go: add workspace mode](https://github.com/golang/go/pull/101112)
   - 状态：Merged
   - 作者：Bob Wilson
   - 描述：添加工作区模式支持
"""

MOCK_GENERATED_CONTENT = """
# golang/go 更新摘要

本次更新主要包含以下重要变更：

1. **编译器优化**
   - 改进了函数内联机制
   - 预期将提升代码执行性能

2. **HTTP 性能提升**
   - 优化了服务器性能
   - 提高了请求处理效率

3. **新特性提案**
   - 考虑在方法上支持类型参数
   - 正在讨论中，可能在未来版本实现

4. **工具链改进**
   - 新增工作区模式
   - 提升了多模块项目的开发体验

## 影响评估

- 性能优化：编译器和 HTTP 服务器的改进将带来显著的性能提升
- 开发体验：工作区模式的加入将改善多模块项目的管理
- 语言特性：方法类型参数的提案将增强语言的表达能力

## 建议关注

- 如果你的项目依赖 HTTP 服务器性能，建议关注相关更新
- 如果你在开发多模块项目，可以尝试新的工作区模式
"""


def get_mock_data():
    """获取模拟数据"""
    return {
        "original_data": MOCK_ORIGINAL_DATA.strip(),
        "generated_content": MOCK_GENERATED_CONTENT.strip(),
    }


def get_mock_models():
    """获取可用的模型列表"""
    return ["llama3.2", "llama2", "gpt-4"]


def get_mock_source_types():
    """获取可用的源类型"""
    return ["github", "gitlab", "bitbucket"]
