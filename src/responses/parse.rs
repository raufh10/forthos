use crate::responses::models::ResponseResponse;
use serde::de::DeserializeOwned;

impl ResponseResponse {
  pub fn get_content(&self) -> Option<String> {
    self.output
      .first()?
      .content
      .first()
      .map(|c| c.text.clone())
  }

  pub fn parse_json<T: DeserializeOwned>(&self) -> Option<Result<T, serde_json::Error>> {
    self.get_content().map(|json_str| serde_json::from_str::<T>(&json_str))
  }

  pub fn get_cached_tokens(&self) -> u32 {
    self.usage
      .as_ref()
      .and_then(|u| u.input_tokens_details.as_ref())
      .map(|d| d.cached_tokens)
      .unwrap_or(0)
  }

  pub fn get_reasoning_tokens(&self) -> u32 {
    self.usage
      .as_ref()
      .and_then(|u| u.output_tokens_details.as_ref())
      .map(|d| d.reasoning_tokens)
      .unwrap_or(0)
  }
}

#[cfg(test)]
mod tests {
  use crate::responses::models::*;
  use serde_json::json;

  #[test]
  fn test_parse_structured_content() {
    let raw_json = json!({
      "output": [{
        "content": [{
          "text": "{\"name\": \"Gemini\"}"
        }]
      }],
      "usage": {
        "input_tokens": 100,
        "input_tokens_details": { "cached_tokens": 50 },
        "output_tokens": 20,
        "output_tokens_details": { "reasoning_tokens": 10 },
        "total_tokens": 120
      }
    });

    let resp: ResponseResponse = serde_json::from_value(raw_json).unwrap();
    assert_eq!(resp.get_content().unwrap(), "{\"name\": \"Gemini\"}");
    assert_eq!(resp.get_cached_tokens(), 50);
    assert_eq!(resp.get_reasoning_tokens(), 10);
  }
}
