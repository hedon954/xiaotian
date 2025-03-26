"""
Xiaotian API 接口

该模块通过 PyO3 与 Rust 后端交互，提供实际数据处理功能
"""
import os
import tempfile
from typing import Dict, List, Tuple, Optional, Union, Any

try:
    from xiaotian_py import _lowlevel

    XIAOTIAN_AVAILABLE = True
except ImportError:
    print("警告: xiaotian_py 模块未找到，将使用模拟数据")
    XIAOTIAN_AVAILABLE = False
    from src.utils.mock import get_mock_data, get_mock_models, get_mock_source_types

# 全局变量，存储处理器实例
_processor = None


def _get_processor():
    """获取或初始化处理器实例"""
    global _processor
    if XIAOTIAN_AVAILABLE and _processor is None:
        try:
            _processor = _lowlevel.Processor()
        except Exception as e:
            print(f"初始化 Xiaotian 处理器失败: {e}")
            return None
    return _processor


def get_available_models() -> List[str]:
    """获取可用的 LLM 模型列表"""
    if not XIAOTIAN_AVAILABLE:
        return get_mock_models()

    processor = _get_processor()
    if processor is None:
        return get_mock_models()

    try:
        return processor.get_model_list()
    except Exception as e:
        print(f"获取模型列表失败: {e}")
        return get_mock_models()


def get_available_source_types() -> List[str]:
    """获取可用的源类型列表"""
    if not XIAOTIAN_AVAILABLE:
        return get_mock_source_types()

    try:
        # 将整数类型映射为字符串
        source_types = _lowlevel.get_source_type_list()
        # 目前只有 GitHub (1)
        source_type_map = {1: "github"}
        return [source_type_map.get(st, "unknown") for st in source_types]
    except Exception as e:
        print(f"获取源类型列表失败: {e}")
        return get_mock_source_types()


def get_available_sources() -> List[str]:
    """获取可用的源列表"""
    if not XIAOTIAN_AVAILABLE:
        return ["golang/go", "rust-lang/rust", "python/cpython"]

    processor = _get_processor()
    if processor is None:
        return ["golang/go", "rust-lang/rust", "python/cpython"]

    try:
        # 返回 (id, name) 的列表，我们只需要名称
        sources = processor.get_source_list()
        return [name for _, name in sources]
    except Exception as e:
        print(f"获取源列表失败: {e}")
        return ["golang/go", "rust-lang/rust", "python/cpython"]


def get_source_id_by_name(name: str) -> Optional[int]:
    """根据源名称获取源 ID"""
    if not XIAOTIAN_AVAILABLE:
        # 模拟数据，返回一个假 ID
        return 1

    processor = _get_processor()
    if processor is None:
        return None

    try:
        sources = processor.get_source_list()
        for source_id, source_name in sources:
            if source_name == name:
                return source_id
        return None
    except Exception as e:
        print(f"获取源 ID 失败: {e}")
        return None


def fetch_updates(model: str, source_type: str, source: str) -> Dict[str, str]:
    """
    获取更新内容

    Args:
        model: 使用的 LLM 模型名称
        source_type: 源类型，如 "github"
        source: 源名称，如 "golang/go"

    Returns:
        包含原始数据和生成内容的字典
    """
    if not XIAOTIAN_AVAILABLE:
        from src.utils.mock import get_mock_data

        return get_mock_data()

    processor = _get_processor()
    if processor is None:
        from src.utils.mock import get_mock_data

        return get_mock_data()

    try:
        # 源类型映射
        source_type_map = {"github": 1}  # 目前只支持 GitHub
        source_type_id = source_type_map.get(source_type.lower(), 1)

        # 获取源 ID
        source_id = get_source_id_by_name(source)
        if source_id is None:
            print(f"未找到源: {source}")
            from src.utils.mock import get_mock_data

            return get_mock_data()

        # 调用后端进行更新获取
        # 由于当前后端实现限制，这里不传递邮箱列表
        result = processor.fetch_updates(source_type_id, source_id, model, [])

        # TODO: 从结果中解析出原始数据和生成内容
        # 当前 fetch_updates 只返回了状态字符串，需要从文件或其他地方获取实际内容
        # 这里暂时使用模拟数据
        from src.utils.mock import get_mock_data

        return get_mock_data()
    except Exception as e:
        print(f"获取更新失败: {e}")
        from src.utils.mock import get_mock_data

        return get_mock_data()


def send_email(content: str, emails: str) -> str:
    """
    发送邮件

    Args:
        content: 要发送的内容
        emails: 邮箱地址，多个邮箱用换行分隔

    Returns:
        发送结果消息
    """
    if not XIAOTIAN_AVAILABLE:
        email_list = [e.strip() for e in emails.split("\n") if e.strip()]
        return f"模拟发送邮件到: {', '.join(email_list)}" if email_list else "请至少提供一个邮箱地址"

    processor = _get_processor()
    if processor is None:
        email_list = [e.strip() for e in emails.split("\n") if e.strip()]
        return f"模拟发送邮件到: {', '.join(email_list)}" if email_list else "请至少提供一个邮箱地址"

    try:
        # 解析邮箱列表
        email_list = [e.strip() for e in emails.split("\n") if e.strip()]
        if not email_list:
            return "请至少提供一个邮箱地址"

        # TODO: 实现通过 Rust 后端发送邮件的功能
        # 当前 PyO3 绑定中没有直接提供发送邮件的方法，需要扩展
        return f"已尝试发送邮件到: {', '.join(email_list)}"
    except Exception as e:
        print(f"发送邮件失败: {e}")
        return f"发送邮件失败: {str(e)}"


def download_content(content: str, format: str = "md") -> str:
    """
    将内容转换为可下载的文件

    Args:
        content: 要下载的内容
        format: 文件格式，目前仅支持 md

    Returns:
        临时文件路径
    """
    if not content:
        return None

    try:
        # 创建临时文件
        fd, temp_path = tempfile.mkstemp(suffix=f".{format}")

        # 写入内容
        with os.fdopen(fd, "w") as f:
            f.write(content)

        return temp_path
    except Exception as e:
        print(f"创建下载文件失败: {e}")
        return None
