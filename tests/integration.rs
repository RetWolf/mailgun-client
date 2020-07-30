#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::env;

    #[tokio::test]
    async fn test_mailgun_send() {
        dotenv().ok();
        let domain =
            env::var("MAILGUN_DOMAIN").expect("Need MAILGUN_DOMAIN set in environment for testing");

        let mailer = mailgun_client::MailgunClient::new(
            domain.clone(),
            mailgun_client::MailgunRegion::US,
            env::var("MAILGUN_API_KEY")
                .expect("Need MAILGUN_API_KEY set in environment for testing"),
        );

        let message = mailgun_client::messages::MailgunMessage::new(
            format!("Mailgun User <mailgun@{}>", domain),
            env::var("EMAIL_TARGET").expect("Need EMAIL_TARGET set in environment for testing"),
        )
        .subject("hello world".to_string())
        .text("testing mailgun api".to_string())
        .build();

        let res = match mailer.send_email(&message).await {
            Ok(r) => r,
            Err(_) => panic!("Error making request"),
        };

        let success_status_code = 200;
        assert_eq!(success_status_code, res.status());

        let api_response = res
            .json::<mailgun_client::messages::MessageAPIResponse>()
            .await
            .unwrap();

        let success_message = "Queued. Thank you.";
        assert_eq!(success_message, api_response.message);
    }
}
