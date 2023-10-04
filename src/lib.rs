#![doc = include_str!("../README.md")]
use connector::Connector;
use error::{SiteVerifyErrors, TurnstileError};
use hyper::{
    client::Client as HyperClient,
    header::{CONTENT_TYPE, USER_AGENT},
    Body, Method, Request,
};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};

mod connector;
pub mod error;

#[cfg(test)]
mod test;

/// A client for the Cloudflare Turnstile API.
pub struct TurnstileClient {
    secret: Secret<String>,
    http: HyperClient<Connector>,
}

/// Represents a request to the Turnstile API.
///
/// <https://developers.cloudflare.com/turnstile/get-started/server-side-validation/#accepted-parameters>
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SiteVerifyRequest {
    /// The secret key for the Turnstile API.
    pub secret: Option<String>,
    /// The response token from the client.
    pub response: String,
    /// The remote IP address of the client providing the respose.
    #[serde(rename = "remote_ip")]
    pub remote_ip: Option<String>,
    /// The idempotency key for the request.
    #[cfg(feature = "idempotency")]
    pub idempotency_key: Option<uuid::Uuid>,
}

/// Represents a succerssful response from the Turnstile API.
///
/// <https://developers.cloudflare.com/turnstile/get-started/server-side-validation/#error-codes:~:text=Successful%20validation%20response>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteVerifyResponse {
    /// Whether the request was successful.
    pub success: bool,
    /// The timestamp of the request.
    #[serde(rename = "challenge_ts")]
    pub timestamp: String,
    /// The hostname of the request.
    pub hostname: String,
    /// The action that was invoked by the turnstile.
    pub action: String,
    /// Data provided by the client.
    pub cdata: String,
}

impl From<RawSiteVerifyResponse> for SiteVerifyResponse {
    fn from(raw: RawSiteVerifyResponse) -> Self {
        Self {
            success: raw.success,
            timestamp: raw.timestamp.unwrap_or_default(),
            hostname: raw.hostname.unwrap_or_default(),
            action: raw.action.unwrap_or_default(),
            cdata: raw.cdata.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RawSiteVerifyResponse {
    success: bool,
    #[serde(rename = "challenge_ts")]
    timestamp: Option<String>,
    hostname: Option<String>,
    #[serde(rename = "error-codes")]
    error_codes: SiteVerifyErrors,
    action: Option<String>,
    cdata: Option<String>,
}

const TURNSTILE_USER_AGENT: &str = concat!(
    "cf-turnstile (",
    env!("CARGO_PKG_HOMEPAGE"),
    ", ",
    env!("CARGO_PKG_VERSION"),
    ")",
);

impl TurnstileClient {
    /// Create a new Turnstile client.
    pub fn new(secret: Secret<String>) -> Self {
        let connector = connector::create();
        let http = hyper::Client::builder().build(connector);

        Self { http, secret }
    }

    /// Verify a Cloudflare Turnstile response.
    pub async fn siteverify(
        &self,
        request: SiteVerifyRequest,
    ) -> Result<SiteVerifyResponse, TurnstileError> {
        // if request secret is none, set it:
        let request = if request.secret.is_none() {
            SiteVerifyRequest {
                secret: Some(self.secret.expose_secret().clone()),
                ..request
            }
        } else {
            request.clone()
        };

        let body = Body::from(serde_json::to_string(&request)?);

        let request = Request::builder()
            .method(Method::POST)
            .uri("https://challenges.cloudflare.com/turnstile/v0/siteverify")
            .header(USER_AGENT, TURNSTILE_USER_AGENT)
            .header(CONTENT_TYPE, "application/json")
            .body(body)
            .expect("request builder");

        let response = self.http.request(request).await?;

        let body_bytes = hyper::body::to_bytes(response).await?;
        let body = serde_json::from_slice::<RawSiteVerifyResponse>(&body_bytes)?;

        if !body.error_codes.is_empty() {
            return Err(TurnstileError::SiteVerifyError(body.error_codes));
        }

        let transformed = SiteVerifyResponse::from(body);

        Ok(transformed)
    }
}

/// Generate a new idempotency key.
#[cfg(feature = "idempotency")]
pub fn generate_indepotency_key() -> Option<uuid::Uuid> {
    Some(uuid::Uuid::new_v4())
}
