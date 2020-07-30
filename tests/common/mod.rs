use dotenv::dotenv;
use mailgun_client::MailgunClient;
use std::env;

pub fn setup_client() -> MailgunClient {
    dotenv().ok();
    let domain =
        env::var("MAILGUN_DOMAIN").expect("Need MAILGUN_DOMAIN set in environment for testing");

    mailgun_client::MailgunClient::new(
        domain.clone(),
        mailgun_client::MailgunRegion::US,
        env::var("MAILGUN_API_KEY").expect("Need MAILGUN_API_KEY set in environment for testing"),
    )
}
