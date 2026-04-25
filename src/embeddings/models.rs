use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum EmbeddingModel {
  #[serde(rename = "text-embedding-ada-002")]
  Ada002,
  #[serde(rename = "text-embedding-3-small")]
  ThreeSmall,
  #[serde(rename = "text-embedding-3-large")]
  ThreeLarge,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum EmbeddingInput {
  String(String),
  ArrayString(Vec<String>),
  ArrayInt(Vec<i32>),
  ArrayArrayInt(Vec<Vec<i32>>),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum EncodingFormat {
  #[serde(rename = "float")]
  Float,
  #[serde(rename = "base64")]
  Base64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingResponse {
  pub data: Vec<EmbeddingData>,
  pub model: String,
  pub object: String,
  pub usage: EmbeddingUsage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingData {
  pub embedding: Vec<f32>,
  pub index: u32,
  pub object: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingUsage {
  pub prompt_tokens: u32,
  pub total_tokens: u32,
}
