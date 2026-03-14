use std::fmt;

/// Unified error type for all cloud provider operations.
///
/// `CloudError` provides a consistent error interface across AWS, GCP, Azure,
/// and other providers, allowing callers to handle errors uniformly regardless
/// of the underlying cloud platform.
#[derive(Debug)]
pub enum CloudError {
    /// Authentication or authorization failure (e.g. expired credentials).
    Auth {
        message: String,
    },
    /// The request was throttled by the provider.
    RateLimit {
        /// Suggested wait time in seconds before retrying, if provided.
        retry_after: Option<u64>,
    },
    /// A provider-specific error with HTTP status information.
    Provider {
        http_status: u16,
        message: String,
        /// Whether this error is safe to retry (e.g. 503 Service Unavailable).
        retryable: bool,
    },
    /// A network-level error (DNS failure, timeout, connection refused, etc.).
    Network {
        source: reqwest::Error,
    },
    /// Failed to serialize or deserialize a request/response payload.
    Serialization {
        source: serde_json::Error,
    },
    /// The requested feature is not supported by this provider.
    Unsupported {
        feature: &'static str,
    },
}

impl fmt::Display for CloudError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CloudError::Auth { message } => write!(f, "authentication error: {}", message),
            CloudError::RateLimit {
                retry_after: Some(s),
            } => {
                write!(f, "rate limit exceeded, retry after {}s", s)
            }
            CloudError::RateLimit { retry_after: None } => write!(f, "rate limit exceeded"),
            CloudError::Provider {
                http_status,
                message,
                ..
            } => {
                write!(f, "provider error {}: {}", http_status, message)
            }
            CloudError::Network { source } => write!(f, "network error: {}", source),
            CloudError::Serialization { source } => write!(f, "serialization error: {}", source),
            CloudError::Unsupported { feature } => write!(f, "unsupported: {}", feature),
        }
    }
}

impl std::error::Error for CloudError {}
