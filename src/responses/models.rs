use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ResponseModel {
  #[serde(rename = "gpt-5.4")]
  Gpt5_4,
  #[serde(rename = "gpt-5.4-mini")]
  Gpt5_4Mini,
  #[serde(rename = "gpt-5.4-nano")]
  Gpt5_4Nano,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum CacheRetention {
  InMemory,
  #[serde(rename = "24h")]
  TwentyFourHours,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseResponse {
  pub output: Vec<Output>,
  pub usage: Option<ResponseUsage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
  pub content: Vec<Content>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Content {
  pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseUsage {
  pub input_tokens: u32,
  pub input_tokens_details: Option<InputTokenDetails>,
  pub output_tokens: u32,
  pub output_tokens_details: Option<OutputTokenDetails>,
  pub total_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct InputTokenDetails {
  #[serde(default)]
  pub cached_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OutputTokenDetails {
  #[serde(default)]
  pub reasoning_tokens: u32,
}
