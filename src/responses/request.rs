use serde::{Serialize, Deserialize};
use crate::responses::models::{ResponseModel, CacheRetention};
use crate::responses::input::ResponseInput;
use crate::responses::text::Text;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseRequest {
  pub model: ResponseModel,
  pub input: ResponseInput,
  pub text: Text,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub prompt_cache_retention: Option<CacheRetention>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub prompt_cache_key: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub service_tier: Option<String>,
}

impl ResponseRequest {
  pub fn new(model: ResponseModel, input: ResponseInput, text: Text) -> Self {
    Self {
      model,
      input,
      text,
      prompt_cache_retention: None,
      prompt_cache_key: None,
      service_tier: None,
    }
  }

  pub fn with_retention(mut self, policy: CacheRetention) -> Self {
    self.prompt_cache_retention = Some(policy);
    self
  }

  pub fn with_cache_key(mut self, key: String) -> Self {
    self.prompt_cache_key = Some(key);
    self
  }

  pub fn with_flex_tier(mut self) -> Self {
    self.service_tier = Some("flex".to_string());
    self
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::responses::models::*;
  use crate::responses::input::*;
  use crate::responses::text::*;
  use serde_json::json;

  #[test]
  fn test_response_request_serialization() {
    let msg = EasyInputMessage {
      content: "Extract names".to_string(),
      role: Role::User,
      phase: Some("initial".to_string()),
      r#type: "text".to_string(),
    };

    let schema = json!({
      "type": "object",
      "properties": { "name": { "type": "string" } }
    });

    let text_config = Text::with_json("user_schema", schema);
    let req = ResponseRequest::new(
      ResponseModel::Gpt5_4,
      ResponseInput::EasyInput(vec![msg]),
      text_config,
    );

    let j = serde_json::to_value(&req).unwrap();

    assert_eq!(j["model"], "gpt-5.4");
    assert_eq!(j["text"]["format"]["type"], "json_schema");
    assert_eq!(j["text"]["format"]["strict"], true);
    assert_eq!(j["input"][0]["role"], "user");
  }

  #[test]
  fn test_response_request_flex_tier() {
    let msg = EasyInputMessage::new_user("Hello");
    let text_config = Text::default();

    let req = ResponseRequest::new(
      ResponseModel::Gpt5_4,
      ResponseInput::EasyInput(vec![msg]),
      text_config,
    ).with_flex_tier();

    let j = serde_json::to_value(&req).unwrap();
    assert_eq!(j["service_tier"], "flex");
  }
}

