use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EmbeddingResponse {
  pub data: Vec<EmbeddingData>,
  pub model: String,
  pub object: String,
  pub usage: EmbeddingUsage,
}

#[derive(Deserialize, Debug)]
pub struct EmbeddingData {
  pub embedding: Vec<f32>,
  pub index: u32,
  pub object: String,
}

#[derive(Deserialize, Debug)]
pub struct EmbeddingUsage {
  pub prompt_tokens: u32,
  pub total_tokens: u32,
}

impl EmbeddingResponse {
  pub fn get_vectors(&self) -> Vec<Vec<f32>> {
    self.data.iter().map(|d| d.embedding.clone()).collect()
  }

  pub fn usage_stats(&self) -> (u32, u32) {
    (self.usage.prompt_tokens, self.usage.total_tokens)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_embedding_response_parsing() {
    let raw = serde_json::json!({
      "object": "list",
      "data": [{
        "object": "embedding",
        "index": 0,
        "embedding": [0.1, 0.2, 0.3]
      }],
      "model": "text-embedding-3-small",
      "usage": { "prompt_tokens": 5, "total_tokens": 5 }
    });

    let res: EmbeddingResponse = serde_json::from_value(raw).unwrap();
    assert_eq!(res.data[0].embedding, vec![0.1, 0.2, 0.3]);
    assert_eq!(res.get_vectors()[0], vec![0.1, 0.2, 0.3]);
  }
}
