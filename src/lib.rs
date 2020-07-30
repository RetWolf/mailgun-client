pub mod messages;

#[derive(PartialEq)]
pub enum MailgunRegion {
    US,
    EU,
}

pub struct MailgunClient {
    pub base_url: String,
    pub domain: String,
    pub api_key: String,
    pub region: MailgunRegion,
}

impl MailgunClient {
    pub fn new(domain: String, region: MailgunRegion, api_key: String) -> MailgunClient {
        if region == MailgunRegion::EU {
            MailgunClient {
                base_url: "https://api.eu.mailgun.net/v3".to_string(),
                domain,
                api_key,
                region,
            }
        } else {
            MailgunClient {
                base_url: "https://api.mailgun.net/v3".to_string(),
                domain,
                api_key,
                region,
            }
        }
    }

    fn build_api_url(&self, api_endpoint: &str) -> String {
        // base_url/domain/api_endpoint
        format!("{}/{}/{}", &self.base_url, &self.domain, api_endpoint)
    }

    pub async fn send_email(
        &self,
        message: &crate::messages::MailgunMessage,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let api_endpoint = "messages";

        let client = reqwest::Client::new();

        let res = client
            .post(&self.build_api_url(api_endpoint))
            .form(&message)
            .basic_auth("api", Some(&self.api_key))
            .send()
            .await?;

        Ok(res)
    }
}
