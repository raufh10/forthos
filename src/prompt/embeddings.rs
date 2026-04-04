use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddingModel {
  #[serde(rename = "text-embedding-ada-002")]
  Ada002,
  #[serde(rename = "text-embedding-3-small")]
  ThreeSmall,
  #[serde(rename = "text-embedding-3-large")]
  ThreeLarge,
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum EmbeddingInput {
  String(String),
  ArrayString(Vec<String>),
  ArrayInt(Vec<i32>),
  ArrayArrayInt(Vec<Vec<i32>>),
}

#[derive(Serialize, Debug)]
pub enum EncodingFormat {
  #[serde(rename = "float")]
  Float,
  #[serde(rename = "base64")]
  Base64,
}

#[derive(Serialize, Debug)]
pub struct EmbeddingRequest {
  pub model: EmbeddingModel,
  pub input: EmbeddingInput,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub dimensions: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub encoding_format: Option<EncodingFormat>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user: Option<String>,
}

impl EmbeddingRequest {
  pub fn new(model: EmbeddingModel, input: EmbeddingInput) -> Self {
    Self {
      model,
      input,
      dimensions: None,
      encoding_format: Some(EncodingFormat::Float),
      user: None,
    }
  }

  pub fn with_dimensions(mut self, dims: u32) -> Self {
    self.dimensions = Some(dims);
    self
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_embedding_input_variants() {
    // Test Single String
    let req_str = EmbeddingRequest::new(
      EmbeddingModel::ThreeSmall,
      EmbeddingInput::String("hello".to_string())
    );
    let j_str = serde_json::to_value(&req_str).unwrap();
    assert!(j_str["input"].is_string());

    // Test Array of Strings
    let req_arr = EmbeddingRequest::new(
      EmbeddingModel::ThreeLarge,
      EmbeddingInput::ArrayString(vec!["a".into(), "b".into()])
    ).with_dimensions(1024);
    
    let j_arr = serde_json::to_value(&req_arr).unwrap();
    assert!(j_arr["input"].is_array());
    assert_eq!(j_arr["dimensions"], 1024);
  }
}
