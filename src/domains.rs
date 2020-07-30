use crate::MailgunClient;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct MailgunDomain {
    pub created_at: String,
    pub id: String,
    pub is_disabled: bool,
    pub name: String,
    pub require_tls: bool,
    pub skip_verification: bool,
    pub smtp_login: String,
    pub smtp_password: String,
    pub spam_action: String,
    pub state: String,
    #[serde(rename = "type")]
    pub domain_type: String,
    pub web_prefix: String,
    pub web_scheme: String,
    pub wildcard: bool,
}

#[derive(Serialize, Deserialize)]
pub struct NewDomain {
    pub name: String,
    pub smtp_password: String,
    pub spam_action: Option<String>,
    pub wildcard: Option<bool>,
    pub force_dkim_authority: Option<bool>,
    pub dkim_key_size: Option<u16>,
    pub ips: Option<String>,
    pub web_scheme: Option<String>,
}

impl NewDomain {
    pub fn new(name: String, smtp_password: String) -> Self {
        Self {
            name,
            smtp_password,
            spam_action: None,
            wildcard: None,
            force_dkim_authority: None,
            dkim_key_size: None,
            ips: None,
            web_scheme: None,
        }
    }

    opt_builder!(spam_action, String);
    opt_builder!(wildcard, bool);
    opt_builder!(force_dkim_authority, bool);
    opt_builder!(dkim_key_size, u16);
    opt_builder!(ips, String);
    opt_builder!(web_scheme, String);
}

#[derive(Serialize, Deserialize)]
pub struct RecievingDNSRecord {
    pub priority: String,
    pub record_type: String,
    pub valid: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct SendingDNSRecord {
    pub record_type: String,
    pub valid: String,
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct DomainsFilter {
    pub authority: Option<String>,
    pub state: Option<String>,
    pub limit: Option<i32>,
    pub skip: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct GetDomainsResponse {
    pub items: Vec<MailgunDomain>,
    pub total_count: i32,
}

#[derive(Serialize, Deserialize)]
pub struct GetDomainResponse {
    pub domain: MailgunDomain,
    pub recieving_dns_records: Option<Vec<RecievingDNSRecord>>,
    pub sending_dns_records: Option<Vec<SendingDNSRecord>>,
}

impl MailgunClient {
    pub async fn get_domains(
        &self,
        filter: Option<DomainsFilter>,
    ) -> Result<GetDomainsResponse, reqwest::Error> {
        let api_endpoint = "domains";

        let client = reqwest::Client::new();

        client
            .get(format!("{}/{}", &self.base_url, api_endpoint).as_str())
            .basic_auth("api", Some(&self.api_key))
            .query(&filter)
            .send()
            .await?
            .json::<GetDomainsResponse>()
            .await
    }

    pub async fn get_domain(&self, domain: String) -> Result<GetDomainResponse, reqwest::Error> {
        let api_endpoint = format!("domains/{}", domain);

        let client = reqwest::Client::new();

        let res = client
            .get(format!("{}/{}", &self.base_url, api_endpoint).as_str())
            .basic_auth("api", Some(&self.api_key))
            .send()
            .await?;

        if res.status() != 200 {
            return Err(res.error_for_status().expect_err("expected error"));
        }

        res.json::<GetDomainResponse>().await
    }
}
