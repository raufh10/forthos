use crate::client::OpenAIClient;
use crate::prompt::responses::ResponseRequest;
use crate::prompt::embeddings::EmbeddingRequest;
use crate::parser::responses::ResponseResponse;
use crate::parser::embeddings::EmbeddingResponse;

pub struct LlmExecutor;

impl LlmExecutor {
  pub async fn execute_response(
    client: &OpenAIClient,
    payload: ResponseRequest,
  ) -> Result<ResponseResponse, reqwest::Error> {
    let url = "https://api.openai.com/v1/responses";

    client
      .http
      .post(url)
      .json(&payload)
      .send()
      .await?
      .json::<ResponseResponse>()
      .await
  }

  pub async fn execute_embedding(
    client: &OpenAIClient,
    payload: EmbeddingRequest,
  ) -> Result<EmbeddingResponse, reqwest::Error> {
    let url = "https://api.openai.com/v1/embeddings";

    client
      .http
      .post(url)
      .json(&payload)
      .send()
      .await?
      .json::<EmbeddingResponse>()
      .await
  }
}

