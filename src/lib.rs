#[macro_use]
mod common;

pub mod domains;
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
}
