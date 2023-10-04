//! https://developers.cloudflare.com/turnstile/reference/testing/
use crate::{
    error::{SiteVerifyError, TurnstileError},
    SiteVerifyRequest, TurnstileClient,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[tokio::test]
async fn test_success() -> Result<()> {
    let client = TurnstileClient::new("1x0000000000000000000000000000000AA".to_string().into());

    let validated = client
        .siteverify(SiteVerifyRequest {
            response: "myresponse".to_string(),
            ..Default::default()
        })
        .await?;

    assert!(validated.success);

    Ok(())
}

#[tokio::test]
async fn test_fail() -> Result<()> {
    let client = TurnstileClient::new("2x0000000000000000000000000000000AA".to_string().into());

    let validated = client
        .siteverify(SiteVerifyRequest {
            response: "myresponse".to_string(),
            ..Default::default()
        })
        .await;

    assert!(validated.is_err());

    Ok(())
}

#[tokio::test]
async fn test_token_already_spent() -> Result<()> {
    let client = TurnstileClient::new("3x0000000000000000000000000000000AA".to_string().into());

    let validated = client
        .siteverify(SiteVerifyRequest {
            response: "myresponse".to_string(),
            ..Default::default()
        })
        .await;

    assert!(validated.is_err());
    match validated.unwrap_err() {
        TurnstileError::SiteVerifyError(e) => match e.first().unwrap() {
            SiteVerifyError::TimeoutOrDuplicate => {},
            _ => panic!("Unexpected error"),
        },
        e => panic!("Unexpected error: {}", e),
    }

    Ok(())
}

// cargo test --features integration -- --nocapture
#[cfg(feature = "integration")]
#[tokio::test]
async fn test_integration() -> Result<()> {
    use crate::generate_indepotency_key;
    use std::env::var;

    let secret_key = var("TURNSTILE_SECRET_KEY").expect("TURNSTILE_SECRET_KEY not set");
    let response = var("TURNSTILE_RESPONSE").expect("TURNSTILE_RESPONSE not set");
    let hostname = var("TURNSTILE_HOSTNAME").expect("TURNSTILE_HOSTNAME not set");
    let idempotency_key = generate_indepotency_key();

    let client = TurnstileClient::new(secret_key.into());

    let validated = client
        .siteverify(SiteVerifyRequest {
            response,
            idempotency_key,
            ..Default::default()
        })
        .await?;

    assert!(validated.success);
    assert_eq!(validated.hostname, hostname);

    println!("validated: {:#?}", validated);

    Ok(())
}
