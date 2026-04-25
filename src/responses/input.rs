use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
  Developer,
  System,
  #[default]
  User,
  Assistant,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ResponseInput {
  TextInput(String),
  EasyInput(Vec<EasyInputMessage>),
}
