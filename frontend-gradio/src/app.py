"""
XiaoTian Gradio 应用入口
"""
import os
from dotenv import load_dotenv
from src.components.interface import create_interface


def main():
    """主函数"""
    # 加载环境变量
    load_dotenv()

    # 创建界面
    interface = create_interface()

    # 启动应用
    interface.launch(
        server_name="localhost",  # 允许外部访问
        server_port=int(os.getenv("PORT", 7860)),  # 默认端口 7860
        share=bool(os.getenv("SHARE", False)),  # 是否创建公开链接
    )


if __name__ == "__main__":
    main()
