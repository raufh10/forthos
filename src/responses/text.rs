use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Verbosity {
  Low,
  #[default]
  Medium,
  High,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
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
    schema: serde_json::Value,
  },
}

#[allow(dead_code)]
fn default_strict() -> bool { true }

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Text {
  pub verbosity: Verbosity,
  pub format: Format,
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

