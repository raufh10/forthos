use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

#[derive(Debug)]
pub struct OpenAIClient {
  pub http: reqwest::Client,
  pub api_key: String,
}

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

    Ok(Self { http, api_key })
  }

  fn build_headers(api_key: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let auth_val = format!("Bearer {}", api_key);

    headers.insert(
      AUTHORIZATION,
      HeaderValue::from_str(&auth_val).expect("Failed to create Auth header")
    );
    headers.insert(
      CONTENT_TYPE,
      HeaderValue::from_static("application/json")
    );
    
    headers
  }

  fn build_http_client(headers: HeaderMap) -> reqwest::Client {
    reqwest::Client::builder()
      .default_headers(headers)
      .build()
      .expect("Failed to build reqwest client")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new_client_valid_key() {
    let res = OpenAIClient::new("sk-12345".to_string());
    assert!(res.is_ok());
    assert_eq!(res.unwrap().api_key, "sk-12345");
  }

  #[test]
  fn test_new_client_empty_key() {
    let res = OpenAIClient::new("  ".to_string());
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "OpenAI API key cannot be empty");
  }

  #[test]
  fn test_new_client_invalid_prefix() {
    let res = OpenAIClient::new("bad-key-123".to_string());
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("should start with 'sk-'"));
  }

  #[test]
  fn test_headers_construction() {
    let key = "sk-test";
    let headers = OpenAIClient::build_headers(key);
    
    assert_eq!(
      headers.get(AUTHORIZATION).unwrap(),
      &format!("Bearer {}", key)
    );
    assert_eq!(
      headers.get(CONTENT_TYPE).unwrap(),
      "application/json"
    );
  }
}
