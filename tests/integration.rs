mod common;

use std::env;

#[tokio::test]
async fn mailgun_send() {
    let client = common::setup_client();
    let message = mailgun_client::messages::MailgunMessage::new(
        format!("Mailgun User <mailgun@{}>", client.domain),
        env::var("EMAIL_TARGET").expect("Need EMAIL_TARGET set in environment for testing"),
    )
    .subject("hello world".to_string())
    .text("testing mailgun api".to_string())
    .build();

    let res = match client.send_email(&message).await {
        Ok(r) => r,
        Err(_) => panic!("Error making request"),
    };

    let success_message = "Queued. Thank you.";
    assert_eq!(success_message, res.message);
}

#[tokio::test]
async fn get_domains_no_filter() {
    let client = common::setup_client();

    let res = match client.get_domains(None).await {
        Ok(r) => r,
        Err(e) => panic!("Error making request: {}", e),
    };

    let correct_total_count = 1;
    assert_eq!(correct_total_count, res.total_count);
}

#[tokio::test]
async fn get_domains_active_filter() {
    let client = common::setup_client();

    let filter = mailgun_client::domains::DomainsFilter {
        authority: None,
        state: Some("active".to_string()),
        limit: None,
        skip: None,
    };

    let res = match client.get_domains(Some(filter)).await {
        Ok(r) => r,
        Err(e) => panic!("Error making request: {}", e),
    };

    let correct_total_count = 1;
    assert_eq!(correct_total_count, res.total_count);

    assert_eq!(client.domain, res.items.first().unwrap().name);
}

#[tokio::test]
async fn get_domain() {
    let client = common::setup_client();

    let res = match client.get_domain(client.domain.clone()).await {
        Ok(r) => r,
        Err(e) => panic!("Error making request: {}", e),
    };

    let state = "active";
    assert_eq!(state, res.domain.state);

    let domain_type = "sandbox";
    assert_eq!(domain_type, res.domain.domain_type);

    let dns_record_len = 0;
    match res.sending_dns_records {
        Some(r) => assert_eq!(dns_record_len, r.len()),
        None => (),
    }
}
