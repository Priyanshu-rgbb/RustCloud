use serde::{Deserialize, Serialize};

use crate::errors::CloudError;

/// Specifies which model to use for an LLM request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelRef {
    /// A provider-specific model identifier (e.g. `"gpt-4"`, `"claude-3"`).
    Provider(String),
    /// A logical model reference by family and optional tier.
    Logical {
        family: String,
        tier: Option<String>,
    },
    /// A deployment-specific identifier (e.g. an Azure deployment name).
    Deployment(String),
}

/// A single message in a conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// The role of the message author (e.g. `"user"`, `"assistant"`, `"system"`).
    pub role: String,
    /// The text content of the message.
    pub content: String,
}

/// A request to an LLM provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmRequest {
    /// The model to target.
    pub model: ModelRef,
    /// The conversation messages.
    pub messages: Vec<Message>,
    /// Optional maximum number of tokens to generate.
    pub max_tokens: Option<u32>,
    /// Optional sampling temperature (0.0 = deterministic, higher = more creative).
    pub temperature: Option<f32>,
    /// Optional system prompt prepended to the conversation.
    pub system_prompt: Option<String>,
}

/// Why the model stopped generating tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FinishReason {
    /// The model produced a natural stop token.
    Stop,
    /// The response was truncated due to max token limit.
    Length,
    /// The model wants to invoke a tool/function.
    ToolCall,
    /// A provider-specific reason.
    Other(String),
}

/// Token usage statistics for a completed request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStats {
    /// Number of tokens in the prompt.
    pub prompt_tokens: u32,
    /// Number of tokens in the completion.
    pub completion_tokens: u32,
}

/// The response from a non-streaming LLM completion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    /// The generated text.
    pub text: String,
    /// Why generation stopped.
    pub finish_reason: FinishReason,
    /// Token usage, if provided by the model.
    pub usage: Option<UsageStats>,
}

/// An event emitted during streaming LLM generation.
#[derive(Debug)]
pub enum LlmStreamEvent {
    /// An incremental text chunk.
    DeltaText(String),
    /// Cumulative token usage statistics.
    Usage(UsageStats),
    /// The stream has completed.
    Done(FinishReason),
    /// An error occurred during streaming.
    Error(CloudError),
}

/// An embedding response containing vectors for each input text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedResponse {
    /// One embedding vector per input text, in the same order.
    pub embeddings: Vec<Vec<f32>>,
}

/// A tool (function) that the model can choose to invoke.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// The name of the tool.
    pub name: String,
    /// A natural-language description of what the tool does.
    pub description: String,
    /// JSON Schema describing the tool's parameters.
    pub parameters: serde_json::Value,
}

/// The response from a tool-calling LLM request.
#[derive(Debug)]
pub enum ToolCallResponse {
    /// The model produced a text response instead of calling a tool.
    Text(LlmResponse),
    /// The model wants to call the specified tool with the given arguments.
    ToolCall {
        name: String,
        arguments: serde_json::Value,
    },
}
