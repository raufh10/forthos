use forthos::{LlmExecutor, OpenAIClient};
use forthos::responses::{ResponseResponse, ResponseRequest, ResponseInput, ResponseModel, Text, Role, EasyInputMessage};
use forthos::embeddings::{EmbeddingResponse, EmbeddingRequest, EmbeddingModel, EmbeddingInput};
use forthos::client::InferenceConfig;

use serde_json::json;
use dotenvy::dotenv;
use std::env;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

fn setup_client() -> OpenAIClient {
  let _ = dotenv(); 
  let key = env::var("OPENAI_API_KEY")
    .expect("OPENAI_API_KEY must be set in .env or environment");

  OpenAIClient::new(key).expect("Failed to initialize OpenAIClient")
}

#[tokio::test]
async fn test_yaml_config_loading_and_execution() {
  let mut client = setup_client();
  let dir = tempdir().unwrap();
  let file_path = dir.path().join("config.yaml");

  // Fully compliant 2026 YAML structure
  let yaml_data = r#"
path: "test_path"
prompts:
  embeddings:
    - model: text-embedding-3-small
      input: "Testing actual API connectivity"
      dimensions: 256
  responses:
    - model: gpt-5.4-mini
      input: "Say hello"
      text:
        verbosity: low
        format:
          type: text
"#;

  let mut file = File::create(&file_path).unwrap();
  file.write_all(yaml_data.as_bytes()).unwrap();

  // 1. Test Loading
  let config = InferenceConfig::from_yaml(&file_path)
    .expect("Module failed to load valid YAML file");

  client = client.with_config(config);

  // 2. Test Actual Calling (Embedding)
  let emb_res: Result<EmbeddingResponse, String> = client.run_embedding_at(0).await;
  assert!(emb_res.is_ok(), "Actual API Call (Embedding) failed: {:?}", emb_res.err());

  // 3. Test Actual Calling (Response)
  let resp_res: Result<ResponseResponse, String> = client.run_response_at(0).await;
  assert!(resp_res.is_ok(), "Actual API Call (Response) failed: {:?}", resp_res.err());
}

#[tokio::test]
async fn test_structured_output_from_yaml_mock() {
  let mut client = setup_client();
  let dir = tempdir().unwrap();
  let file_path = dir.path().join("structured.yaml");

  let yaml_data = r#"
path: "structured_test"
prompts:
  embeddings: []
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

  let mut file = File::create(&file_path).unwrap();
  file.write_all(yaml_data.as_bytes()).unwrap();

  let config = InferenceConfig::from_yaml(&file_path).expect("Failed to load YAML");
  client = client.with_config(config);

  // 4. Test Calling + Structured Output Parsing
  let result: ResponseResponse = client.run_response_at(0).await.expect("API Call failed");

  #[derive(serde::Deserialize, Debug)]
  struct Person { name: String, age: u32 }

  let person = result.parse_json::<Person>()
    .expect("No content found")
    .expect("Failed to parse JSON schema output");

  assert_eq!(person.name, "John");
}

#[tokio::test]
async fn test_invalid_yaml_schema_validation() {
  let dir = tempdir().unwrap();
  let file_path = dir.path().join("invalid.yaml");

  // Added 'verbosity' and 'embeddings' so it passes the Parser and hits the Validator
  let invalid_yaml = r#"
path: "invalid_test"
prompts:
  embeddings: []
  responses:
    - model: gpt-5.4-mini
      input: "test"
      text:
        verbosity: low
        format:
          type: json_schema
          name: "Invalid Name With Spaces"
          schema:
            type: object
"#;

  let mut file = File::create(&file_path).unwrap();
  file.write_all(invalid_yaml.as_bytes()).unwrap();

  let result = InferenceConfig::from_yaml(&file_path);

  // 5. Test Validation Logic
  assert!(result.is_err(), "Validator should have caught the space in the schema name");
  let err_msg = result.unwrap_err().to_lowercase();
  assert!(err_msg.contains("invalid") || err_msg.contains("schema"), "Wrong error message: {}", err_msg);
}

