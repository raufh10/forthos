use crate::embeddings::models::EmbeddingResponse;

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
  use crate::embeddings::models::EmbeddingResponse;

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
