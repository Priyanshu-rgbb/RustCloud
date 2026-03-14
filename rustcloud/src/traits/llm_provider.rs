use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;

use crate::errors::CloudError;
use crate::types::llm::{
    EmbedResponse, LlmRequest, LlmResponse, LlmStreamEvent, ToolCallResponse, ToolDefinition,
};

/// A pinned, boxed stream of LLM events used for streaming completions.
pub type LlmStream = Pin<Box<dyn Stream<Item = LlmStreamEvent> + Send>>;

/// Trait for interacting with Large Language Model providers.
///
/// Implement this trait for each cloud provider (e.g. AWS Bedrock, GCP Vertex AI)
/// to enable a unified interface for text generation, streaming, embeddings, and
/// tool-calling across providers.
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Generate a single completion from the model.
    async fn generate(&self, req: LlmRequest) -> Result<LlmResponse, CloudError>;

    /// Stream completion tokens from the model as they are produced.
    async fn stream(&self, req: LlmRequest) -> Result<LlmStream, CloudError>;

    /// Generate vector embeddings for the given texts.
    async fn embed(&self, texts: Vec<String>) -> Result<EmbedResponse, CloudError>;

    /// Generate a completion that may include tool/function calls.
    async fn generate_with_tools(
        &self,
        req: LlmRequest,
        tools: Vec<ToolDefinition>,
    ) -> Result<ToolCallResponse, CloudError>;
}
