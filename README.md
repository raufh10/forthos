# forthos 🦀
**forthos** is a Rust wrapper for the OpenAI **v1/responses** and **embeddings** endpoints. It simplifies working with structured outputs, prompt caching, and vector embeddings.
## 📦 Installation
Add this to your Cargo.toml:
```toml
[dependencies]
forthos = "0.1.0"
tokio = { version = "1", features = ["full"] }
serde_json = "1.0"

```
## 🚀 Quick Start
### 1. Structured Output (JSON Schema)
Extracting structured data is a first-class citizen in **forthos**.
```rust
use forthos::{OpenAIClient, LlmExecutor, Role, create_message};
use forthos::responses::{ResponseRequest, ResponseInput, ResponseModel, Text};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = OpenAIClient::new("your-api-key")?;

  // Define your schema using serde_json
  let schema = json!({
    "type": "object",
    "properties": {
      "capital": { "type": "string" }
    },
    "required": ["capital"]
  });

  // Create message: role, content, phase (Option), type (Option)
  let msg = create_message(Role::User, "What is the capital of France?", None, None);
  
  let request = ResponseRequest::new(
    ResponseModel::Gpt5_4Mini,
    ResponseInput::EasyInput(vec![msg]),
    Text::with_json("location_schema", schema),
  );

  let response = LlmExecutor::execute_response(&client, request).await?;
  
  if let Some(content) = response.get_content() {
    println!("Response: {}", content); // {"capital": "Paris"}
  }

  Ok(())
}

```
### 2. Prompt Caching
Save costs on repetitive prompts by utilizing built-in caching support.
```rust
use forthos::responses::CacheRetention;

let request = ResponseRequest::new(model, input, text)
  .with_retention(CacheRetention::TwentyFourHours)
  .with_cache_key("unique-session-id".to_string());

```
### 3. Embeddings
Generate vectors with specific dimensions easily.
```rust
use forthos::embeddings::{EmbeddingRequest, EmbeddingModel, EmbeddingInput};

let request = EmbeddingRequest::new(
  EmbeddingModel::ThreeSmall,
  EmbeddingInput::String("Rust programming".to_string()),
).with_dimensions(512);

let res = LlmExecutor::execute_embedding(&client, request).await?;
println!("Vector size: {}", res.data[0].embedding.len());

```
## 📄 License
Licensed under the **MIT License**. See LICENSE for more information.
