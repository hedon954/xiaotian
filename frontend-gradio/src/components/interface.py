"""
Gradio 界面组件
"""
import gradio as gr
from src.utils.mock import get_mock_data, get_mock_models, get_mock_source_types


def create_interface():
    """创建 Gradio 界面"""

    def fetch_updates(model, source_type, source):
        """获取更新（占位函数）"""
        # TODO: 实际实现将通过 FFI 调用 Rust 代码
        data = get_mock_data()
        return data["original_data"], data["generated_content"]

    def send_email(content, emails):
        """发送邮件（占位函数）"""
        # TODO: 实际实现将通过 FFI 调用 Rust 代码
        email_list = [e.strip() for e in emails.split("\n") if e.strip()]
        if not email_list:
            return "请至少提供一个邮箱地址"
        return f"已将内容发送至：{', '.join(email_list)}"

    # 获取选项数据
    models = get_mock_models()
    source_types = get_mock_source_types()
    sources = ["golang/go", "rust-lang/rust", "python/cpython"]  # 示例数据

    # 创建界面
    with gr.Blocks(
        css="""
        .input-container {
            min-height: 100px !important;
            margin-bottom: 10px !important;
        }
        .content-container-left {
            height: 500px !important;
            background-color: #f3f4f6 !important;
            border-radius: 8px !important;
            padding: 10px !important;
            overflow-y: auto !important;
        }
        .content-container-right {
            height: 483px !important;
            background-color: #f3f4f6 !important;
            border-radius: 8px !important;
            padding: 10px !important;
            overflow-y: auto !important;
        }
        .button-row {
            margin: 10px 0 !important;
        }
        .email-input {
            height: 60px !important;
            overflow-y: auto !important;
        }
    """
    ) as interface:
        with gr.Row():
            # 左侧面板
            with gr.Column(scale=1):
                # 输入区域
                with gr.Column(elem_classes="input-container"):
                    with gr.Row():
                        gr.Markdown("model")
                        gr.Markdown("source_type")
                        gr.Markdown("source")

                    with gr.Row():
                        model = gr.Dropdown(
                            choices=models,
                            value=models[0] if models else None,
                            interactive=True,
                            show_label=False,
                            scale=1,
                        )
                        source_type = gr.Dropdown(
                            choices=source_types,
                            value=source_types[0] if source_types else None,
                            interactive=True,
                            show_label=False,
                            scale=1,
                        )
                        source = gr.Dropdown(
                            choices=sources,
                            value=sources[0] if sources else None,
                            interactive=True,
                            show_label=False,
                            scale=1,
                        )

                with gr.Row(elem_classes="button-row"):
                    fetch_btn = gr.Button("fetch_updates")

                gr.Markdown("original data (markdown)")
                with gr.Column(elem_classes="content-container-left"):
                    original_data = gr.Markdown()

            # 右侧面板
            with gr.Column(scale=1):
                # 输入区域
                with gr.Column(elem_classes="input-container"):
                    gr.Markdown("email")
                    emails = gr.Textbox(
                        show_label=False,
                        placeholder="输入邮箱地址，按回车键确认添加多个",
                        lines=3,
                        max_lines=5,
                        container=False,
                        elem_classes="email-input",
                    )

                with gr.Row(elem_classes="button-row"):
                    send_btn = gr.Button("sendTo")

                with gr.Row():
                    gr.Markdown("generated content (markdown)")
                    download_btn = gr.Button("download", scale=0)

                with gr.Column(elem_classes="content-container-right"):
                    generated_content = gr.Markdown()

        # 事件处理
        fetch_btn.click(
            fn=fetch_updates,
            inputs=[model, source_type, source],
            outputs=[original_data, generated_content],
        )

        send_btn.click(fn=send_email, inputs=[generated_content, emails], outputs=[])

        # 下载功能
        download_btn.click(
            fn=lambda x: x, inputs=[generated_content], outputs=[gr.File()]
        )

    return interface
