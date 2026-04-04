pub mod embeddings;
pub mod responses;

pub use embeddings::{
  EmbeddingData, 
  EmbeddingResponse, 
  EmbeddingUsage
};

pub use responses::{
  Content,
  InputTokenDetails,
  Output,
  OutputTokenDetails,
  ResponseResponse,
  ResponseUsage,
};

pub trait OpenAIResponse {
  fn total_tokens(&self) -> u32;
}

impl OpenAIResponse for EmbeddingResponse {
  fn total_tokens(&self) -> u32 {
    self.usage.total_tokens
  }
}

impl OpenAIResponse for ResponseResponse {
  fn total_tokens(&self) -> u32 {
    self.usage.as_ref().map(|u| u.total_tokens).unwrap_or(0)
  }
}
