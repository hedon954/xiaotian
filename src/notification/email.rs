use mail_send::{SmtpClientBuilder, mail_builder::MessageBuilder};
use pulldown_cmark::{Parser, html};
use serde::{Deserialize, Serialize};

use super::{NotificationError, NotificationMessage, Notifier};
use async_trait::async_trait;

/// 邮件配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    /// SMTP服务器地址
    pub smtp_server: String,
    /// SMTP服务器端口
    pub smtp_port: u16,
    /// SMTP用户名
    pub username: String,
    /// SMTP密码
    pub password: String,
    /// 发件人地址
    pub from: String,
    /// 收件人地址列表
    pub to: Vec<String>,
    /// 是否使用TLS
    pub use_tls: bool,
}

/// 邮件通知器
pub struct EmailNotifier {
    config: EmailConfig,
}

impl EmailNotifier {
    pub fn new(config: EmailConfig) -> Self {
        Self { config }
    }

    fn markdown_to_html(&self, markdown: &str) -> String {
        let parser = Parser::new(markdown);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }
}

#[async_trait]
impl Notifier for EmailNotifier {
    async fn send(&self, message: &NotificationMessage) -> Result<(), NotificationError> {
        // 将Markdown转换为HTML
        let html_content = self.markdown_to_html(&message.content);

        // 解析收件人地址
        let to: Vec<String> = self.config.to.clone();

        // 构建邮件
        let email = MessageBuilder::new()
            .from(self.config.from.clone())
            .to(to)
            .subject(&message.subject)
            .html_body(html_content); // 使用转换后的HTML内容

        SmtpClientBuilder::new(self.config.smtp_server.clone(), self.config.smtp_port)
            .implicit_tls(true)
            .credentials((self.config.username.clone(), self.config.password.clone()))
            .connect()
            .await
            .map_err(|e| NotificationError::ChannelUnavailable(e.to_string()))?
            .send(email)
            .await
            .map_err(|e| NotificationError::SendFailed(e.to_string()))?;

        Ok(())
    }
}
