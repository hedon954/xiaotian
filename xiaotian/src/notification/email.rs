use mail_send::{SmtpClientBuilder, mail_builder::MessageBuilder};
use pulldown_cmark::{Parser, html};
use serde::{Deserialize, Serialize};

use super::{NotificationError, NotificationMessage, Notifier};
use async_trait::async_trait;

/// Email configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    /// SMTP server address
    pub smtp_server: String,
    /// SMTP server port
    pub smtp_port: u16,
    /// SMTP username
    pub username: String,
    /// SMTP password
    pub password: String,
    /// sender address
    pub from: String,
    /// recipient addresses
    pub to: Vec<String>,
    /// whether to use TLS
    pub use_tls: bool,
}

/// Email notifier
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
    async fn send(
        &self,
        message: &NotificationMessage,
        to: Vec<String>,
    ) -> Result<(), NotificationError> {
        let email = MessageBuilder::new()
            .from(self.config.from.clone())
            .to(to)
            .subject(&message.subject)
            .html_body(self.markdown_to_html(&message.content));

        SmtpClientBuilder::new(self.config.smtp_server.clone(), self.config.smtp_port)
            .implicit_tls(self.config.use_tls)
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
