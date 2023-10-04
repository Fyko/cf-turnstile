//! Error types for the Turnstile API.
use serde::{Deserialize, Serialize};
use thiserror::Error;
// ​​Error codes
// Error code	Description
// missing-input-secret	The secret parameter was not passed.
// invalid-input-secret	The secret parameter was invalid or did not exist.
// missing-input-response	The response parameter was not passed.
// invalid-input-response	The response parameter is invalid or has expired.
// invalid-widget-id	The widget ID extracted from the parsed site secret key was invalid or did not exist.
// invalid-parsed-secret	The secret extracted from the parsed site secret key was invalid.
// bad-request	The request was rejected because it was malformed.
// timeout-or-duplicate	The response parameter has already been validated before.
// internal-error	An internal error happened while validating the response. The request can be retried.

/// Represents a list of errors from the Turnstile API.
#[derive(Debug, Error)]
pub enum TurnstileError {
    /// The error originated from the Turnstile API.
    #[error("Turnstile API error: {0:?}")]
    SiteVerifyError(SiteVerifyErrors),

    /// The error originated from Hyper.
    #[error("Hyper error: {0:?}")]
    HyperError(#[from] hyper::Error),

    /// The error originated from Serde.
    #[error("Serde error: {0:?}")]
    SerdeError(#[from] serde_json::Error),
}

/// Represents a list of errors from the Turnstile API.
pub type SiteVerifyErrors = Vec<SiteVerifyError>;

/// Represents an error from the Turnstile API.
///
/// <https://developers.cloudflare.com/turnstile/get-started/server-side-validation/#error-codes>
#[derive(Debug, Clone, Error, Deserialize, Serialize)]
pub enum SiteVerifyError {
    /// The secret parameter was not passed.
    #[serde(rename = "missing-input-secret")]
    #[error("The secret parameter was not passed.")]
    MissingInputSecret,

    /// The secret parameter was invalid or did not exist.
    #[serde(rename = "invalid-input-secret")]
    #[error("The secret parameter was invalid or did not exist.")]
    InvalidInputSecret,

    /// The response parameter was not passed.
    #[serde(rename = "missing-input-response")]
    #[error("The response parameter was not passed.")]
    MissingInputResponse,

    /// The response parameter is invalid or has expired.
    #[serde(rename = "invalid-input-response")]
    #[error("The response parameter is invalid or has expired.")]
    InvalidInputResponse,

    /// The widget ID extracted from the parsed site secret key was invalid or did not exist.
    #[serde(rename = "invalid-widget-id")]
    #[error(
        "The widget ID extracted from the parsed site secret key was invalid or did not exist."
    )]
    InvalidWidgetId,

    /// The secret extracted from the parsed site secret key was invalid.
    #[serde(rename = "invalid-parsed-secret")]
    #[error("The secret extracted from the parsed site secret key was invalid.")]
    InvalidParsedSecret,

    /// The request was rejected because it was malformed.
    #[serde(rename = "bad-request")]
    #[error("The request was rejected because it was malformed.")]
    BadRequest,

    /// The response parameter has already been validated before.
    #[serde(rename = "timeout-or-duplicate")]
    #[error("The response parameter has already been validated before.")]
    TimeoutOrDuplicate,

    /// An internal error happened while validating the response. The request can be retried.
    #[serde(rename = "internal-error")]
    #[error(
        "An internal error happened while validating the response. The request can be retried."
    )]
    InternalError,
}
