"""
XiaoTian Gradio 应用入口
"""
import os
from dotenv import load_dotenv
from src.components.interface import create_interface


def main():
    """Main function"""
    # load environment variables
    load_dotenv()

    # create interface
    interface = create_interface()

    # launch application
    interface.launch(
        server_name="localhost",  # allow external access
        server_port=int(os.getenv("PORT", 7860)),  # default port 7860
        share=bool(os.getenv("SHARE", False)),  # whether to create a public link
    )


if __name__ == "__main__":
    main()
