use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Debug)]
pub enum ResponseModel {
  #[serde(rename = "gpt-5.4")]
  Gpt5_4,
  #[serde(rename = "gpt-5.4-mini")]
  Gpt5_4Mini,
  #[serde(rename = "gpt-5.4-nano")]
  Gpt5_4Nano,
}

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum Role {
  Developer,
  System,
  #[default]
  User,
  Assistant,
}

#[derive(Serialize, Debug)]
pub struct EasyInputMessage {
  pub content: String,
  pub role: Role,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub phase: Option<String>,
  pub r#type: String,
}

impl EasyInputMessage {
  pub fn new_user(content: &str) -> Self {
    Self {
      content: content.to_string(),
      role: Role::User,
      phase: None,
      r#type: "message".to_string(),
    }
  }

  pub fn new_user_with_phase(content: &str, phase: &str) -> Self {
    Self {
      content: content.to_string(),
      role: Role::User,
      phase: Some(phase.to_string()),
      r#type: "message".to_string(),
    }
  }
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum ResponseInput {
  TextInput(String),
  EasyInput(Vec<EasyInputMessage>),
}

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum Verbosity {
  Low,
  #[default]
  Medium,
  High,
}

#[derive(Serialize, Debug, Default, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Format {
  #[default]
  Text,
  JsonSchema {
    name: String,
    #[serde(default = "default_strict")]
    strict: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    schema: Value,
  },
}

#[allow(dead_code)]
fn default_strict() -> bool { true }

#[derive(Serialize, Debug, Default)]
pub struct Text {
  pub verbosity: Verbosity,
  pub format: Format,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum CacheRetention {
  InMemory,
  #[serde(rename = "24h")]
  TwentyFourHours,
}

#[derive(Serialize, Debug)]
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

impl Text {
  pub fn new(verbosity: Verbosity, format: Format) -> Self {
    Self { verbosity, format }
  }

  pub fn with_json(name: &str, schema: Value) -> Self {
    Self {
      verbosity: Verbosity::Medium,
      format: Format::JsonSchema {
        name: name.to_string(),
        strict: true,
        description: None,
        schema,
      },
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
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
    
    // Verify specific OpenAI requirements
    assert_eq!(j["model"], "gpt-5.4");
    assert_eq!(j["text"]["format"]["type"], "json_schema");
    assert_eq!(j["text"]["format"]["strict"], true);
    assert_eq!(j["input"][0]["role"], "user");
  }

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
