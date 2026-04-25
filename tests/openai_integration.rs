use forthos::{LlmExecutor, OpenAIClient};
use forthos::responses::{ResponseResponse, ResponseRequest, ResponseInput, ResponseModel, Text, Role, EasyInputMessage};
use forthos::embeddings::{EmbeddingResponse, EmbeddingRequest, EmbeddingModel, EmbeddingInput};
use forthos::client::InferenceConfig;

use serde_json::json;
use dotenvy::dotenv;
use std::env;
use std::io::Write;
use tempfile::NamedTempFile;

fn setup_client() -> OpenAIClient {
  let _ = dotenv(); 
  let key = env::var("OPENAI_API_KEY")
    .expect("OPENAI_API_KEY must be set in .env or environment");

  OpenAIClient::new(key).expect("Failed to initialize OpenAIClient")
}

#[tokio::test]
async fn test_yaml_config_loading_and_execution() {
  let mut client = setup_client();

  let yaml_data = r#"
path: "test_path"
prompts:
  embeddings:
    - model: text-embedding-3-small
      input: "Mocking YAML is easy"
      dimensions: 256
  responses:
    - model: gpt-5.4-mini
      input: "Say hello"
      text:
        verbosity: low
        format:
          type: text
"#;

  let mut temp_file = NamedTempFile::new().unwrap();
  writeln!(temp_file, "{}", yaml_data).unwrap();

  let config = InferenceConfig::from_yaml(temp_file.path())
    .expect("Failed to parse YAML mock data");

  client = client.with_config(config);

  let emb_res: Result<EmbeddingResponse, String> = client.run_embedding_at(0).await;
  assert!(emb_res.is_ok(), "YAML Embedding failed: {:?}", emb_res.err());

  let resp_res: Result<ResponseResponse, String> = client.run_response_at(0).await;
  assert!(resp_res.is_ok(), "YAML Response failed: {:?}", resp_res.err());
}

#[tokio::test]
async fn test_structured_output_from_yaml_mock() {
  let mut client = setup_client();

  let yaml_data = r#"
path: "structured_test"
prompts:
  responses:
    - model: gpt-5.4-mini
      input:
        - role: user
          content: "Generate a person: John, 30."
          type: message
      text:
        verbosity: high
        format:
          type: json_schema
          name: person_schema
          strict: true
          schema:
            type: object
            properties:
              name: { type: string }
              age: { type: integer }
            required: ["name", "age"]
            additionalProperties: false
"#;

  let config: InferenceConfig = yaml_serde::from_str(yaml_data)
    .expect("Failed to deserialize YAML string");

  client = client.with_config(config);

  let result: ResponseResponse = client.run_response_at(0).await.expect("API Call failed");

  #[derive(serde::Deserialize, Debug)]
  struct Person { name: String, age: u32 }

  let person = result.parse_json::<Person>()
    .expect("No content found")
    .expect("Failed to parse JSON schema output");

  assert_eq!(person.name, "John");
  assert_eq!(person.age, 30);
}

#[tokio::test]
async fn test_invalid_yaml_schema_validation() {
  let invalid_yaml = r#"
path: "invalid_test"
prompts:
  responses:
    - model: gpt-5.4-mini
      input: "test"
      text:
        format:
          type: json_schema
          name: "Invalid Name With Spaces"
          schema:
            type: object
"#;

  let mut temp_file = NamedTempFile::new().unwrap();
  writeln!(temp_file, "{}", invalid_yaml).unwrap();

  let result = InferenceConfig::from_yaml(temp_file.path());

  assert!(result.is_err(), "Validator should have caught the invalid name");
  
  let err_msg = result.unwrap_err().to_lowercase();
  assert!(err_msg.contains("invalid") || err_msg.contains("schema"), "Actual error was: {}", err_msg);
}

