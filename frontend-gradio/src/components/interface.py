"""
Gradio interface component
"""
import gradio as gr
import time
import tempfile
import os

# try to import Python binding and initialize processor
from xiaotian_py_binding._lowlevel import Processor, get_source_type_list

# create a global processor instance - if failed, it will raise an exception and terminate the program
processor = Processor()
print("successfully initialized Processor")


def create_interface():
    """create Gradio interface"""

    def get_available_models():
        """get available LLM model list"""
        try:
            return processor.get_model_list()
        except Exception as e:
            print(f"get model list failed: {e}")
            return ["llama3.2"]  # default value

    def get_available_source_types():
        """get available source type list"""
        try:
            # map integer type to string
            source_types = get_source_type_list()
            # currently only GitHub (1)
            source_type_map = {1: "github"}
            return [source_type_map.get(st, "unknown") for st in source_types]
        except Exception as e:
            print(f"get source type list failed: {e}")
            return ["github"]  # default value

    def get_available_sources():
        """get available source list"""
        try:
            # return (id, name) list, we only need name
            sources = processor.get_source_list()
            return [name for _, name in sources]
        except Exception as e:
            print(f"get source list failed: {e}")
            return ["golang/go"]  # default value

    def get_source_id_by_name(name):
        """get source id by source name"""
        try:
            sources = processor.get_source_list()
            for source_id, source_name in sources:
                if source_name == name:
                    return source_id
            return None
        except Exception as e:
            print(f"get source id failed: {e}")
            return None

    def fetch_updates_with_progress(
        model, source_type, source, emails="", progress=gr.Progress()
    ):
        """fetch updates with progress"""
        try:
            # initial state
            yield "", "", "<span class='info-message'>⏳ 初始化...</span>"
            time.sleep(0.5)  # add a short delay to make the state visible
            progress(0.2, desc="准备获取仓库更新...")

            # prepare stage
            yield "", "", "<span class='info-message'>⏳ 准备获取仓库更新...</span>"
            time.sleep(0.5)  # add a short delay to make the state visible
            source_type_map = {"github": 1}
            source_type_id = source_type_map.get(source_type.lower(), 1)

            # find source id
            yield "", "", "<span class='info-message'>⏳ 正在查找源ID...</span>"
            time.sleep(0.5)  # add a short delay to make the state visible
            progress(0.3, desc="查找源ID...")
            source_id = get_source_id_by_name(source)
            if source_id is None:
                yield "", "", "<span class='error-message'>✗ 未找到源</span>"
                raise ValueError(f"未找到源: {source}")

            # process email list
            email_list = []
            if emails and emails.strip():
                email_list = [e.strip() for e in emails.split("\n") if e.strip()]
                yield "", "", f"<span class='info-message'>⏳ 将发送更新到: {', '.join(email_list)}</span>"
                time.sleep(0.5)  # add a short delay to make the state visible

            # fetch updates
            yield "", "", f"<span class='info-message'>⏳ 正在获取「{source}」的更新...</span>"
            progress(0.5, desc=f"正在获取「{source}」的更新...")

            # call backend to process
            original_data, generated_content = processor.fetch_updates(
                source_type_id, int(source_id), model, email_list
            )

            time.sleep(0.5)  # add a short delay to make the state visible
            # complete
            yield original_data, generated_content, "<span class='success-message'>✓ 更新获取成功!</span>"

        except Exception as e:
            yield "", "", f"<span class='error-message'>✗ 获取失败: {str(e)}</span>"

    def download_markdown_with_progress(content, progress=gr.Progress()):
        """download with progress"""
        if not content:
            return None

        progress(0, desc="准备下载...")
        try:
            progress(0.3, desc="创建临时文件...")

            # get current timestamp
            current_time = int(time.time())

            # create filename (use default value, because we cannot get current selected source_type and source)
            filename = f"github_golang_go_{current_time}.md"

            # create temporary file
            fd, temp_path = tempfile.mkstemp(suffix=".md")

            progress(0.6, desc="写入内容...")

            # write content
            with os.fdopen(fd, "w") as f:
                f.write(content)

            progress(0.9, desc="准备下载...")
            time.sleep(0.3)  # add a short delay to make the progress visible

            progress(1.0, desc="文件已准备就绪")

            # return single file path instead of tuple
            return temp_path

        except Exception as e:
            progress(1.0, desc=f"创建下载文件失败: {e}")
            print(f"创建下载文件失败: {e}")
            return None

    def send_email_with_progress(content, emails, progress=gr.Progress()):
        """send email with progress"""
        progress(0, desc="准备发送邮件...")

        if not emails or not emails.strip():
            progress(1.0, desc="未提供邮箱地址")
            return "请至少提供一个邮箱地址"

        # parse email list
        email_list = [e.strip() for e in emails.split("\n") if e.strip()]
        if not email_list:
            progress(1.0, desc="未提供有效邮箱地址")
            return "请至少提供一个有效的邮箱地址"

        try:
            progress(0.3, desc="正在连接邮件服务器...")

            # get current selected source and model (from UI)
            # this part may need to be improved, because we cannot get the current UI selected value now

            # source type mapping
            source_type_id = 1  # assume current is GitHub

            # TODO: get current selected source id
            source_id = (
                1  # this needs to be improved, get current selected source id from UI
            )

            # get current selected model
            model = "llama3.2"  # this needs to be improved, get current selected model from UI

            progress(0.5, desc=f"正在发送邮件到 {', '.join(email_list)}...")

            # call binding to send email
            # here we reuse fetch_updates method, but pass email list
            processor.fetch_updates(
                source_type_id,
                source_id,
                model,
                email_list,  # pass actual email list here
            )

            progress(1.0, desc="邮件已发送")
            return f"<span class='success-message'>✓ 已成功发送到: {', '.join(email_list)}</span>"
        except Exception as e:
            progress(0.9, desc=f"发送失败: {e}")
            progress(1.0, desc="发送过程中出现错误")
            return f"<span class='error-message'>✗ 发送失败: {str(e)}</span>"

    def validate_emails(emails_str):
        """validate email address format"""
        if not emails_str or not emails_str.strip():
            return "请输入至少一个邮箱地址"

        email_list = [e.strip() for e in emails_str.split("\n") if e.strip()]
        invalid_emails = []

        for email in email_list:
            # simple email format validation
            if "@" not in email or "." not in email:
                invalid_emails.append(email)

        if invalid_emails:
            return f"<span class='error-message'>以下邮箱格式无效：{', '.join(invalid_emails)}</span>"
        return f"<span class='success-message'>已添加 {len(email_list)} 个邮箱地址</span>"

    # get options data
    models = get_available_models()
    source_types = get_available_source_types()
    sources = get_available_sources()

    # create interface
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
        # status indicator component
        status_indicator = gr.Markdown(visible=False)

        with gr.Row():
            # left panel
            with gr.Column(scale=1):
                # input area
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

            # right panel
            with gr.Column(scale=1):
                # input area
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

        # event handling
        def update_sources(source_type):
            """update source list"""
            # currently API does not support filtering by source type, return all sources
            return gr.Dropdown.update(choices=get_available_sources())

        # update source list when source type changes
        source_type.change(fn=update_sources, inputs=[source_type], outputs=[source])

        # get updates event
        fetch_btn.click(
            fn=fetch_updates_with_progress,
            inputs=[model, source_type, source, emails],
            outputs=[original_data, generated_content, fetch_status],
            queue=True,  # enable queue to support intermediate state updates
        )

        # send email event
        send_btn.click(
            fn=send_email_with_progress,
            inputs=[generated_content, emails],
            outputs=[result_message],
            queue=True,  # enable queue to support intermediate state updates
        )

        # download feature
        download_btn.click(
            fn=download_markdown_with_progress,
            inputs=[generated_content],
            outputs=gr.File(label="下载文件"),  # modify output component
            queue=True,
        )

        # add email validation event
        emails.change(fn=validate_emails, inputs=[emails], outputs=[result_message])

    return interface
