use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ResponseResponse {
  pub output: Vec<Output>,
  pub usage: Option<ResponseUsage>,
}

#[derive(Deserialize, Debug)]
pub struct Output {
  pub content: Vec<Content>,
}

#[derive(Deserialize, Debug)]
pub struct Content {
  pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct ResponseUsage {
  pub input_tokens: u32,
  pub input_tokens_details: Option<InputTokenDetails>,
  pub output_tokens: u32,
  pub output_tokens_details: Option<OutputTokenDetails>,
  pub total_tokens: u32,
}

#[derive(Deserialize, Debug)]
pub struct InputTokenDetails {
  #[serde(default)]
  pub cached_tokens: u32,
}

#[derive(Deserialize, Debug)]
pub struct OutputTokenDetails {
  #[serde(default)]
  pub reasoning_tokens: u32,
}

impl ResponseResponse {
  pub fn get_content(&self) -> Option<String> {
    self.output
      .first()?
      .content
      .first()
      .map(|c| c.text.clone())
  }

  pub fn parse_json<T>(&self) -> Option<Result<T, serde_json::Error>> 
  where 
    for<'de> T: Deserialize<'de> 
  {
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
  use super::*;
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
  }
}
