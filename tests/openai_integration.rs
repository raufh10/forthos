use forthos::{LlmExecutor, OpenAIClient};
use forthos::responses::{ResponseRequest, ResponseInput, ResponseModel, Text, Role, EasyInputMessage};
use forthos::embeddings::{EmbeddingRequest, EmbeddingModel, EmbeddingInput};
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

  // 1. Create a Mock YAML string
  let yaml_data = r#"
path: "test_path"
prompts:
  embeddings:
    - model: text-embedding-3-small
      input: "Mocking YAML is easy"
      dimensions: 256
  responses:
    - model: gpt-5.4-mini
      input:
        text_input: "Say hello"
      text:
        verbosity: low
        format:
          type: text
"#;

  // 2. Create temporary file
  let mut temp_file = NamedTempFile::new().unwrap();
  writeln!(temp_file, "{}", yaml_data).unwrap();

  // 3. Load config using our logic from config_build.rs
  let config = InferenceConfig::from_yaml(temp_file.path())
    .expect("Failed to parse YAML mock data");

  // 4. Inject config into client
  client = client.with_config(config);

  // 5. Test execution by index
  // Test Embedding (Index 0)
  let emb_res = client.run_embedding_at(0).await;
  assert!(emb_res.is_ok(), "YAML Embedding failed: {:?}", emb_res.err());
  assert_eq!(emb_res.unwrap().data[0].embedding.len(), 256);

  // Test Response (Index 0)
  let resp_res = client.run_response_at(0).await;
  assert!(resp_res.is_ok(), "YAML Response failed: {:?}", resp_res.err());
}

#[tokio::test]
async fn test_structured_output_from_yaml_mock() {
  let mut client = setup_client();

  // Mocking JSON Schema in YAML
  let yaml_data = r#"
prompts:
  responses:
    - model: gpt-5.4-mini
      input:
        easy_input:
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

  let result = client.run_response_at(0).await.unwrap();
  
  #[derive(serde::Deserialize, Debug)]
  struct Person { name: String, age: u32 }
  
  let person = result.parse_json::<Person>().unwrap().unwrap();
  
  assert_eq!(person.name, "John");
  assert_eq!(person.age, 30);
}

#[tokio::test]
async fn test_invalid_yaml_schema_validation() {
  // Testing custom validator in config_build.rs
  let invalid_yaml = r#"
prompts:
  responses:
    - model: gpt-5.4-mini
      text:
        format:
          type: json_schema
          name: "Invalid Name With Spaces"
          schema:
            not_an_object: 123
"#;

  let mut temp_file = NamedTempFile::new().unwrap();
  writeln!(temp_file, "{}", invalid_yaml).unwrap();

  let result = InferenceConfig::from_yaml(temp_file.path());
  
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("Invalid schema name"));
}

