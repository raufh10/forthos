use forthos::OpenAIClient;
use forthos::responses::ResponseResponse;
use forthos::embeddings::EmbeddingResponse;
use forthos::client::InferenceConfig;

use dotenvy::dotenv;
use std::env;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;
use std::time::Instant;

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

  let config = InferenceConfig::from_yaml(&file_path)
    .expect("Module failed to load valid YAML file");

  client = client.with_config(config);

  println!("\n--- [START] test_yaml_config_loading_and_execution ---");
  
  let now = Instant::now();
  let emb_res: Result<EmbeddingResponse, String> = client.run_embedding_at(0).await;
  println!("Embedding Latency: {:?}", now.elapsed());
  assert!(emb_res.is_ok(), "Actual API Call (Embedding) failed: {:?}", emb_res.err());
  println!("Embedding Response: Successfully received {} vectors", emb_res.unwrap().data[0].embedding.len());

  let now = Instant::now();
  let resp_res: Result<ResponseResponse, String> = client.run_response_at(0).await;
  println!("Response Latency: {:?}", now.elapsed());
  assert!(resp_res.is_ok(), "Actual API Call (Response) failed: {:?}", resp_res.err());
  
  if let Ok(res) = resp_res {
    // Fixed: converted &str to String for unwrap_or
    println!("Response Content: {}", res.get_content().unwrap_or_else(|| "EMPTY".to_string()));
  }
  
  println!("--- [END] test_yaml_config_loading_and_execution ---\n");
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

  println!("\n--- [START] test_structured_output_from_yaml_mock ---");
  
  let result: ResponseResponse = client.run_response_at(0).await.expect("API Call failed");

  #[derive(serde::Deserialize, Debug)]
  struct Person { 
    name: String, 
    #[serde(rename = "age")] // Map "age" from JSON to "_age" in Rust
    _age: u32 
  }

  let person = result.parse_json::<Person>()
    .expect("No content found")
    .expect("Failed to parse JSON schema output");

  println!("Structured Result: {:?}", person);
  assert_eq!(person.name, "John");
  
  println!("--- [END] test_structured_output_from_yaml_mock ---\n");
}

#[tokio::test]
async fn test_invalid_yaml_schema_validation() {
  let dir = tempdir().unwrap();
  let file_path = dir.path().join("invalid.yaml");

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

  println!("\n--- [START] test_invalid_yaml_schema_validation ---");
  
  assert!(result.is_err());
  let err_msg = result.unwrap_err().to_lowercase();
  println!("Caught Expected Error: {}", err_msg);
  assert!(err_msg.contains("invalid") || err_msg.contains("schema"));
  
  println!("--- [END] test_invalid_yaml_schema_validation ---\n");
}

