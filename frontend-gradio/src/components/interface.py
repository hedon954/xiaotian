"""
Gradio 界面组件
"""
import gradio as gr
import time
import tempfile
import os

# 尝试导入 Python 绑定并初始化处理器
from xiaotian_py_binding._lowlevel import Processor, get_source_type_list

# 创建全局处理器实例 - 如果失败，将会抛出异常并终止程序
processor = Processor()
print("成功初始化 Processor")


def create_interface():
    """创建 Gradio 界面"""

    def get_available_models():
        """获取可用的 LLM 模型列表"""
        try:
            return processor.get_model_list()
        except Exception as e:
            print(f"获取模型列表失败: {e}")
            return ["llama3.2"]  # 默认值

    def get_available_source_types():
        """获取可用的源类型列表"""
        try:
            # 将整数类型映射为字符串
            source_types = get_source_type_list()
            # 目前只有 GitHub (1)
            source_type_map = {1: "github"}
            return [source_type_map.get(st, "unknown") for st in source_types]
        except Exception as e:
            print(f"获取源类型列表失败: {e}")
            return ["github"]  # 默认值

    def get_available_sources():
        """获取可用的源列表"""
        try:
            # 返回 (id, name) 的列表，我们只需要名称
            sources = processor.get_source_list()
            return [name for _, name in sources]
        except Exception as e:
            print(f"获取源列表失败: {e}")
            return ["golang/go"]  # 默认值

    def get_source_id_by_name(name):
        """根据源名称获取源 ID"""
        try:
            sources = processor.get_source_list()
            for source_id, source_name in sources:
                if source_name == name:
                    return source_id
            return None
        except Exception as e:
            print(f"获取源 ID 失败: {e}")
            return None

    def fetch_updates_with_progress(
        model, source_type, source, emails="", progress=gr.Progress()
    ):
        """带有进度显示的更新获取函数"""
        try:
            # 初始状态
            yield "", "", "<span class='info-message'>⏳ 初始化...</span>"
            time.sleep(0.5)  # 添加短暂延时使状态可见
            progress(0.2, desc="准备获取仓库更新...")

            # 准备阶段
            yield "", "", "<span class='info-message'>⏳ 准备获取仓库更新...</span>"
            time.sleep(0.5)  # 添加短暂延时使状态可见
            source_type_map = {"github": 1}
            source_type_id = source_type_map.get(source_type.lower(), 1)

            # 查找源ID
            yield "", "", "<span class='info-message'>⏳ 正在查找源ID...</span>"
            time.sleep(0.5)  # 添加短暂延时使状态可见
            progress(0.3, desc="查找源ID...")
            source_id = get_source_id_by_name(source)
            if source_id is None:
                yield "", "", "<span class='error-message'>✗ 未找到源</span>"
                raise ValueError(f"未找到源: {source}")

            # 处理邮箱列表
            email_list = []
            if emails and emails.strip():
                email_list = [e.strip() for e in emails.split("\n") if e.strip()]
                yield "", "", f"<span class='info-message'>⏳ 将发送更新到: {', '.join(email_list)}</span>"
                time.sleep(0.5)  # 添加短暂延时使状态可见

            # 获取更新
            yield "", "", f"<span class='info-message'>⏳ 正在获取「{source}」的更新...</span>"
            progress(0.5, desc=f"正在获取「{source}」的更新...")

            # 调用后端处理
            original_data, generated_content = processor.fetch_updates(
                source_type_id, int(source_id), model, email_list
            )

            time.sleep(0.5)  # 添加短暂延时使状态可见
            # 完成
            yield original_data, generated_content, "<span class='success-message'>✓ 更新获取成功!</span>"

        except Exception as e:
            yield "", "", f"<span class='error-message'>✗ 获取失败: {str(e)}</span>"

    def download_markdown_with_progress(content, progress=gr.Progress()):
        """带有进度显示的下载函数"""
        if not content:
            return None

        progress(0, desc="准备下载...")
        try:
            progress(0.3, desc="创建临时文件...")

            # 获取当前时间戳
            current_time = int(time.time())

            # 创建文件名（这里使用默认值，因为无法获取当前选择的source_type和source）
            filename = f"github_golang_go_{current_time}.md"

            # 创建临时文件
            fd, temp_path = tempfile.mkstemp(suffix=".md")

            progress(0.6, desc="写入内容...")

            # 写入内容
            with os.fdopen(fd, "w") as f:
                f.write(content)

            progress(0.9, desc="准备下载...")
            time.sleep(0.3)  # 短暂延时让进度可见

            progress(1.0, desc="文件已准备就绪")

            # 返回单个文件路径而不是元组
            return temp_path

        except Exception as e:
            progress(1.0, desc=f"创建下载文件失败: {e}")
            print(f"创建下载文件失败: {e}")
            return None

    def send_email_with_progress(content, emails, progress=gr.Progress()):
        """带有进度显示的邮件发送函数"""
        progress(0, desc="准备发送邮件...")

        if not emails or not emails.strip():
            progress(1.0, desc="未提供邮箱地址")
            return "请至少提供一个邮箱地址"

        # 解析邮箱列表
        email_list = [e.strip() for e in emails.split("\n") if e.strip()]
        if not email_list:
            progress(1.0, desc="未提供有效邮箱地址")
            return "请至少提供一个有效的邮箱地址"

        try:
            progress(0.3, desc="正在连接邮件服务器...")

            # 获取当前选择的源和模型（从UI中）
            # 这部分可能需要改进，因为现在无法获取当前UI选择的值

            # 源类型映射
            source_type_id = 1  # 假设当前是 GitHub

            # TODO: 获取当前选中的源ID
            source_id = 1  # 这个需要改进，从UI获取当前选择的源ID

            # 获取当前选择的模型
            model = "llama3.2"  # 这个也需要改进

            progress(0.5, desc=f"正在发送邮件到 {', '.join(email_list)}...")

            # 调用绑定发送邮件
            # 这里我们重用 fetch_updates 方法，但传递邮箱地址
            processor.fetch_updates(
                source_type_id, source_id, model, email_list  # 这里传递实际的邮箱列表
            )

            progress(1.0, desc="邮件已发送")
            return f"<span class='success-message'>✓ 已成功发送到: {', '.join(email_list)}</span>"
        except Exception as e:
            progress(0.9, desc=f"发送失败: {e}")
            progress(1.0, desc="发送过程中出现错误")
            return f"<span class='error-message'>✗ 发送失败: {str(e)}</span>"

    def validate_emails(emails_str):
        """验证邮箱地址格式"""
        if not emails_str or not emails_str.strip():
            return "请输入至少一个邮箱地址"

        email_list = [e.strip() for e in emails_str.split("\n") if e.strip()]
        invalid_emails = []

        for email in email_list:
            # 简单的邮箱格式验证
            if "@" not in email or "." not in email:
                invalid_emails.append(email)

        if invalid_emails:
            return f"<span class='error-message'>以下邮箱格式无效：{', '.join(invalid_emails)}</span>"
        return f"<span class='success-message'>已添加 {len(email_list)} 个邮箱地址</span>"

    # 获取选项数据
    models = get_available_models()
    source_types = get_available_source_types()
    sources = get_available_sources()

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
        .result-message {
            color: #4f46e5;
            font-size: 14px;
            margin-top: 5px;
        }
        /* 添加按钮加载状态样式 */
        .loading {
            opacity: 0.7;
            cursor: wait !important;
        }
        /* 添加结果消息样式 */
        .success-message {
            color: #10B981;
            font-weight: bold;
        }
        .error-message {
            color: #EF4444;
            font-weight: bold;
        }
        .info-message {
            color: #3B82F6;
            font-weight: bold;
        }
    """
    ) as interface:
        # 状态指示组件
        status_indicator = gr.Markdown(visible=False)

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
                    fetch_status = gr.Markdown(elem_classes="result-message")

                gr.Markdown("original data (markdown)")
                with gr.Column(elem_classes="content-container-left"):
                    original_data = gr.Markdown()

            # 右侧面板
            with gr.Column(scale=1):
                # 输入区域
                with gr.Column(elem_classes="input-container"):
                    gr.Markdown("email (one per line)")
                    emails = gr.Textbox(
                        show_label=False,
                        placeholder="每行输入一个邮箱地址，例如:\nuser1@example.com\nuser2@example.com",
                        lines=3,
                        max_lines=5,
                        container=False,
                        elem_classes="email-input",
                        value="171725713@qq.com",  # 设置默认值
                    )

                with gr.Row(elem_classes="button-row"):
                    send_btn = gr.Button("sendTo")
                    result_message = gr.Markdown(elem_classes="result-message")

                with gr.Row():
                    gr.Markdown("generated content (markdown)")
                    download_btn = gr.Button("download", scale=0)

                with gr.Column(elem_classes="content-container-right"):
                    generated_content = gr.Markdown()

        # 事件处理
        def update_sources(source_type):
            """更新源列表"""
            # 目前 API 不支持按源类型过滤，返回所有源
            return gr.Dropdown.update(choices=get_available_sources())

        # 源类型变更时更新源列表
        source_type.change(fn=update_sources, inputs=[source_type], outputs=[source])

        # 获取更新事件
        fetch_btn.click(
            fn=fetch_updates_with_progress,
            inputs=[model, source_type, source, emails],
            outputs=[original_data, generated_content, fetch_status],
            queue=True,  # 启用队列以支持中间状态更新
        )

        # 发送邮件事件
        send_btn.click(
            fn=send_email_with_progress,
            inputs=[generated_content, emails],
            outputs=[result_message],
            queue=True,  # 启用队列以支持中间状态更新
        )

        # 下载功能
        download_btn.click(
            fn=download_markdown_with_progress,
            inputs=[generated_content],
            outputs=gr.File(label="下载文件"),  # 修改输出组件
            queue=True,
        )

        # 添加邮箱验证事件
        emails.change(fn=validate_emails, inputs=[emails], outputs=[result_message])

    return interface
