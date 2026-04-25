use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use crate::client::models::{OpenAIClient, InferenceConfig};

impl OpenAIClient {
  pub fn new(api_key: String) -> Result<Self, String> {
    if api_key.trim().is_empty() {
      return Err("OpenAI API key cannot be empty".to_string());
    }

    if !api_key.starts_with("sk-") {
      return Err("Invalid OpenAI API key format (should start with 'sk-')".to_string());
    }

    let headers = Self::build_headers(&api_key);
    let http = Self::build_http_client(headers);

    Ok(Self { 
      http, 
      api_key,
      config: InferenceConfig::default(),
    })
  }

  pub fn with_config(mut self, config: InferenceConfig) -> Self {
    self.config = config;
    self
  }

  fn build_headers(api_key: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let auth_val = format!("Bearer {}", api_key);

    headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_val).unwrap());
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers
  }

  fn build_http_client(headers: HeaderMap) -> ClientWithMiddleware {
    let raw_client = reqwest::Client::builder()
      .default_headers(headers)
      .build()
      .expect("Failed to build reqwest client");

    let retry_policy = ExponentialBackoff::builder()
      .build_with_max_retries(3); 

    ClientBuilder::new(raw_client)
      .with(RetryTransientMiddleware::new_with_policy(retry_policy))
      .build()
  }
}

