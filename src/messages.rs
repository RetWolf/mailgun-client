use crate::MailgunClient;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct MailgunMessage {
    /// from: Email address for From header
    /// Must be in the form "{username} <{user}@{domain}>"
    from: String,
    /// to: Email address of the recipient(s). Example: "Bob <bob@host.com>". You can use commas to separate multiple recipients.
    to: String,
    /// cc: Same as To but for Cc
    cc: Option<String>,
    /// bcc: Same as To but for Bcc
    bcc: Option<String>,
    /// subject: Message subject
    subject: Option<String>,
    /// text: Body of the message. (text version)
    text: Option<String>,
    /// html: Body of the message. (HTML version)
    html: Option<String>,
}

impl MailgunMessage {
    pub fn new(from: String, to: String) -> Self {
        Self {
            from: from,
            to: to,
            cc: None,
            bcc: None,
            subject: None,
            text: None,
            html: None,
        }
    }

    opt_builder!(cc, String);
    opt_builder!(bcc, String);
    opt_builder!(subject, String);
    opt_builder!(text, String);
    opt_builder!(html, String);

    pub fn build(&self) -> Self {
        self.clone()
    }
}

#[derive(Serialize, Deserialize)]
pub struct MessageAPIResponse {
    pub id: String,
    pub message: String,
}

impl MailgunClient {
    pub async fn send_email(
        &self,
        message: &MailgunMessage,
    ) -> Result<MessageAPIResponse, reqwest::Error> {
        let api_endpoint = "messages";

        let client = reqwest::Client::new();

        client
            .post(format!("{}/{}/{}", &self.base_url, &self.domain, api_endpoint).as_str())
            .form(&message)
            .basic_auth("api", Some(&self.api_key))
            .send()
            .await?
            .json::<MessageAPIResponse>()
            .await
    }
}
