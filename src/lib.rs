pub mod client;
pub mod embeddings;
pub mod responses;

use crate::client::OpenAIClient;
use crate::responses::{ResponseRequest, ResponseResponse};
use crate::embeddings::{EmbeddingRequest, EmbeddingResponse};

pub struct LlmExecutor;

impl LlmExecutor {
  pub async fn execute_response(
    client: &OpenAIClient,
    payload: &ResponseRequest,
  ) -> Result<ResponseResponse, reqwest_middleware::Error> {
    let url = "https://api.openai.com/v1/responses";

    let body = serde_json::to_value(payload).map_err(|e| {
      reqwest_middleware::Error::Middleware(anyhow::anyhow!(e))
    })?;

    client
      .http
      .post(url)
      .body(body.to_string())
      .header("Content-Type", "application/json")
      .send()
      .await?
      .json::<ResponseResponse>()
      .await
      .map_err(|e| reqwest_middleware::Error::Reqwest(e))
  }

  pub async fn execute_embedding(
    client: &OpenAIClient,
    payload: &EmbeddingRequest,
  ) -> Result<EmbeddingResponse, reqwest_middleware::Error> {
    let url = "https://api.openai.com/v1/embeddings";

    let body = serde_json::to_value(payload).map_err(|e| {
      reqwest_middleware::Error::Middleware(anyhow::anyhow!(e))
    })?;

    client
      .http
      .post(url)
      .body(body.to_string())
      .header("Content-Type", "application/json")
      .send()
      .await?
      .json::<EmbeddingResponse>()
      .await
      .map_err(|e| reqwest_middleware::Error::Reqwest(e))
  }
}

impl OpenAIClient {
  /// Runs a response prompt from the config by index
  pub async fn run_response_at(&self, index: usize) -> Result<ResponseResponse, String> {
    let prompt = self.config.prompts.responses.get(index)
      .ok_or_else(|| format!("Response prompt at index {} not found", index))?;

    LlmExecutor::execute_response(self, prompt)
      .await
      .map_err(|e| e.to_string())
  }

  /// Runs an embedding prompt from the config by index
  pub async fn run_embedding_at(&self, index: usize) -> Result<EmbeddingResponse, String> {
    let prompt = self.config.prompts.embeddings.get(index)
      .ok_or_else(|| format!("Embedding prompt at index {} not found", index))?;

    LlmExecutor::execute_embedding(self, prompt)
      .await
      .map_err(|e| e.to_string())
  }
}
