use async_trait::async_trait;

/// Trait for retrieving authentication tokens from cloud providers.
///
/// Implementations handle credential resolution, token refresh, and caching
/// for their respective provider.
#[async_trait]
pub trait TokenProvider: Send + Sync {
    /// Retrieve a valid authentication token.
    async fn get_token(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
}
