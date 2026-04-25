use serde::{Serialize, Deserialize};
use crate::embeddings::models::{EmbeddingModel, EmbeddingInput, EncodingFormat};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingRequest {
  pub model: EmbeddingModel,
  pub input: EmbeddingInput,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub dimensions: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub encoding_format: Option<EncodingFormat>,
}

impl EmbeddingRequest {
  pub fn new(model: EmbeddingModel, input: EmbeddingInput) -> Self {
    Self {
      model,
      input,
      dimensions: None,
      encoding_format: None,
    }
  }

  pub fn with_dimensions(mut self, dims: u32) -> Result<Self, String> {
    if self.model == EmbeddingModel::Ada002 {
      return Err("Ada-002 does not support custom dimensions".into());
    }
    self.dimensions = Some(dims);
    Ok(self)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::embeddings::models::*;

  #[test]
  fn test_dimensions_validation() {
    let req = EmbeddingRequest::new(
      EmbeddingModel::Ada002,
      EmbeddingInput::String("test".into())
    );
    
    // Should fail for Ada
    assert!(req.with_dimensions(512).is_err());

    let req_v3 = EmbeddingRequest::new(
      EmbeddingModel::ThreeSmall,
      EmbeddingInput::String("test".into())
    );

    // Should fail for 0
    assert!(req_v3.with_dimensions(0).is_err());
    
    // Should pass for v3 with positive int
    let valid_req = EmbeddingRequest::new(
      EmbeddingModel::ThreeLarge,
      EmbeddingInput::String("test".into())
    ).with_dimensions(1024).unwrap();
    
    assert_eq!(valid_req.dimensions, Some(1024));
  }
}

