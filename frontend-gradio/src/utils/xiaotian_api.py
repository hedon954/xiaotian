"""
Xiaotian API interface

This module interacts with the Rust backend through PyO3, providing actual data processing functions
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

# global variable, store processor instance
_processor = None


def _get_processor():
    """Get or initialize processor instance"""
    global _processor
    if XIAOTIAN_AVAILABLE and _processor is None:
        try:
            _processor = _lowlevel.Processor()
        except Exception as e:
            print(f"初始化 Xiaotian 处理器失败: {e}")
            return None
    return _processor


def get_available_models() -> List[str]:
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
    if not XIAOTIAN_AVAILABLE:
        return get_mock_source_types()

    try:
        # map integer type to string
        source_types = _lowlevel.get_source_type_list()
        # currently only GitHub (1)
        source_type_map = {1: "github"}
        return [source_type_map.get(st, "unknown") for st in source_types]
    except Exception as e:
        print(f"get source type list failed: {e}")
        return get_mock_source_types()


def get_available_sources() -> List[str]:
    if not XIAOTIAN_AVAILABLE:
        return ["golang/go", "rust-lang/rust", "python/cpython"]

    processor = _get_processor()
    if processor is None:
        return ["golang/go", "rust-lang/rust", "python/cpython"]

    try:
        # return (id, name) list, we only need name
        sources = processor.get_source_list()
        return [name for _, name in sources]
    except Exception as e:
        print(f"get source list failed: {e}")
        return ["golang/go", "rust-lang/rust", "python/cpython"]


def get_source_id_by_name(name: str) -> Optional[int]:
    if not XIAOTIAN_AVAILABLE:
        # mock data, return a fake ID
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
        print(f"get source id failed: {e}")
        return None


def fetch_updates(model: str, source_type: str, source: str) -> Dict[str, str]:
    """
    Get update content

    Args:
        model: the name of the LLM model
        source_type: the type of the source, e.g. "github"
        source: the name of the source, e.g. "golang/go"

    Returns:
        a dictionary containing the original data and generated content
    """
    if not XIAOTIAN_AVAILABLE:
        from src.utils.mock import get_mock_data

        return get_mock_data()

    processor = _get_processor()
    if processor is None:
        from src.utils.mock import get_mock_data

        return get_mock_data()

    try:
        # source type mapping
        source_type_map = {"github": 1}  # currently only GitHub
        source_type_id = source_type_map.get(source_type.lower(), 1)

        # get source id
        source_id = get_source_id_by_name(source)
        if source_id is None:
            print(f"未找到源: {source}")
            from src.utils.mock import get_mock_data

            return get_mock_data()

        # call backend to fetch updates
        # due to the current backend implementation, we don't pass the email list here
        result = processor.fetch_updates(source_type_id, source_id, model, [])

        # TODO: parse the original data and generated content from the result
        # currently fetch_updates only returns the status string, we need to get the actual content from the file or other places
        # here we use mock data temporarily
        from src.utils.mock import get_mock_data

        return get_mock_data()
    except Exception as e:
        print(f"get updates failed: {e}")
        from src.utils.mock import get_mock_data

        return get_mock_data()


def send_email(content: str, emails: str) -> str:
    """
    Send email

    Args:
        content: the content to send
        emails: the email addresses, multiple emails separated by newlines

    Returns:
        the result message of sending email
    """
    if not XIAOTIAN_AVAILABLE:
        email_list = [e.strip() for e in emails.split("\n") if e.strip()]
        return (
            f"mock send email to: {', '.join(email_list)}"
            if email_list
            else "please provide at least one email address"
        )

    processor = _get_processor()
    if processor is None:
        email_list = [e.strip() for e in emails.split("\n") if e.strip()]
        return (
            f"mock send email to: {', '.join(email_list)}"
            if email_list
            else "please provide at least one email address"
        )

    try:
        # parse email list
        email_list = [e.strip() for e in emails.split("\n") if e.strip()]
        if not email_list:
            return "please provide at least one email address"

        # TODO: implement the function to send email through Rust backend
        # currently the PyO3 binding does not provide a direct method to send email, we need to extend it
        return f"tried to send email to: {', '.join(email_list)}"
    except Exception as e:
        print(f"send email failed: {e}")
        return f"send email failed: {str(e)}"


def download_content(content: str, format: str = "md") -> str:
    """
    Convert content to a downloadable file

    Args:
        content: the content to download
        format: the file format, currently only md is supported

    Returns:
        the path of the temporary file
    """
    if not content:
        return None

    try:
        # create a temporary file
        fd, temp_path = tempfile.mkstemp(suffix=f".{format}")

        # write content to the file
        with os.fdopen(fd, "w") as f:
            f.write(content)

        return temp_path
    except Exception as e:
        print(f"create download file failed: {e}")
        return None
