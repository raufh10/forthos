use serde::{Serialize, Deserialize};
use crate::embeddings::request::EmbeddingRequest;
use crate::responses::request::ResponseRequest;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InferenceConfig {
  pub path: String,
  pub prompts: PromptsConfig,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PromptsConfig {
  pub embeddings: Vec<EmbeddingRequest>,
  pub responses: Vec<ResponseRequest>,
}

#[derive(Debug)]
pub struct OpenAIClient {
  pub http: reqwest::Client,
  pub api_key: String,
  pub config: InferenceConfig,
}

