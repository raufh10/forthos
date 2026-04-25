pub mod client;
pub mod embeddings;
pub mod responses;

pub use crate::client::models::{OpenAIClient, InferenceConfig};
pub use crate::responses::request::ResponseRequest;
pub use crate::responses::models::ResponseResponse;
pub use crate::embeddings::request::EmbeddingRequest;
pub use crate::embeddings::models::EmbeddingResponse;

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

    let res = client
      .http
      .post(url)
      .body(body.to_string())
      .header("Content-Type", "application/json")
      .send()
      .await?;

    let response_data: ResponseResponse = res
      .json::<ResponseResponse>()
      .await
      .map_err(reqwest_middleware::Error::Reqwest)?;

    Ok(response_data)
  }

  pub async fn execute_embedding(
    client: &OpenAIClient,
    payload: &EmbeddingRequest,
  ) -> Result<EmbeddingResponse, reqwest_middleware::Error> {
    let url = "https://api.openai.com/v1/embeddings";

    let body = serde_json::to_value(payload).map_err(|e| {
      reqwest_middleware::Error::Middleware(anyhow::anyhow!(e))
    })?;

    let res = client
      .http
      .post(url)
      .body(body.to_string())
      .header("Content-Type", "application/json")
      .send()
      .await?;

    let embedding_data: EmbeddingResponse = res
      .json::<EmbeddingResponse>()
      .await
      .map_err(reqwest_middleware::Error::Reqwest)?;

    Ok(embedding_data)
  }
}

impl OpenAIClient {
  pub async fn run_response_at(&self, index: usize) -> Result<ResponseResponse, String> {
    let prompt = self.config.prompts.responses.get(index)
      .ok_or_else(|| format!("Response prompt at index {} not found", index))?;

    LlmExecutor::execute_response(self, prompt)
      .await
      .map_err(|e| e.to_string())
  }

  pub async fn run_embedding_at(&self, index: usize) -> Result<EmbeddingResponse, String> {
    let prompt = self.config.prompts.embeddings.get(index)
      .ok_or_else(|| format!("Embedding prompt at index {} not found", index))?;

    LlmExecutor::execute_embedding(self, prompt)
      .await
      .map_err(|e| e.to_string())
  }
}

