"""
界面测试
"""
from src.utils.mock import get_mock_data, get_mock_models, get_mock_source_types


def test_mock_data():
    """测试模拟数据"""
    data = get_mock_data()
    assert "original_data" in data
    assert "generated_content" in data
    assert isinstance(data["original_data"], str)
    assert isinstance(data["generated_content"], str)


def test_mock_models():
    """测试模型列表"""
    models = get_mock_models()
    assert isinstance(models, list)
    assert len(models) > 0
    assert "llama3.2" in models


def test_mock_source_types():
    """测试源类型列表"""
    source_types = get_mock_source_types()
    assert isinstance(source_types, list)
    assert len(source_types) > 0
    assert "github" in source_types
