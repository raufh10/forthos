use forthos::client::OpenAIClient;
use forthos::llm::LlmExecutor;
use forthos::prompt::{create_message, Role};
use forthos::prompt::responses::{
  ResponseRequest, ResponseInput, ResponseModel, Text
};
use forthos::prompt::embeddings::{EmbeddingRequest, EmbeddingModel, EmbeddingInput};
use forthos::parser::responses::ResponseResponse;
use forthos::parser::embeddings::EmbeddingResponse;

use serde_json::json;
use dotenvy::dotenv;
use std::env;

fn setup_client() -> OpenAIClient {
  let _ = dotenv(); 
  let key = env::var("OPENAI_API_KEY")
    .expect("OPENAI_API_KEY must be set in .env or environment");
  OpenAIClient::new(key).expect("Failed to initialize OpenAIClient")
}

#[tokio::test]
async fn test_structured_output_flow() {
  let client = setup_client();

  let msg = create_message(
    Role::User, 
    "What is the capital of France?", 
    None, 
    Some("message")
  );

  let schema = json!({
    "type": "object",
    "properties": {
      "capital": { "type": "string" }
    },
    "required": ["capital"],
    "additionalProperties": false
  });

  let text_config = Text::with_json("location_schema", schema);
  
  let request = ResponseRequest::new(
    ResponseModel::Gpt5_4Mini,
    ResponseInput::EasyInput(vec![msg]),
    text_config,
  );

  let result: Result<ResponseResponse, reqwest::Error> = 
    LlmExecutor::execute_response(&client, request).await;

  assert!(result.is_ok(), "API request failed: {:?}", result.err());
  
  let resp = result.unwrap();
  let content = resp.get_content();
  
  assert!(content.is_some(), "Response content was empty");
  println!("Received Structured Content: {}", content.unwrap());
}

#[tokio::test]
async fn test_embedding_flow() {
  let client = setup_client();

  let request = EmbeddingRequest::new(
    EmbeddingModel::ThreeSmall,
    EmbeddingInput::String("Rust programming is awesome".to_string()),
  ).with_dimensions(512);

  let result: Result<EmbeddingResponse, reqwest::Error> = 
    LlmExecutor::execute_embedding(&client, request).await;

  assert!(result.is_ok(), "Embedding request failed: {:?}", result.err());
  
  let resp = result.unwrap();
  assert!(!resp.data.is_empty());
  assert_eq!(resp.data[0].embedding.len(), 512);
  println!("Successfully retrieved embedding with 512 dimensions");
}
