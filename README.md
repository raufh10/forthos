# forthos 🦀
**forthos** is a configuration-driven Rust wrapper for the OpenAI **v1/responses** and **embeddings** endpoints. It allows you to move all your AI prompt engineering, schema definitions, and model parameters into validated YAML files.
## 📦 Installation
Add this to your Cargo.toml:
```toml
[dependencies]
forthos = "0.2.0"
tokio = { version = "1", features = ["full"] }

```
## 🚀 The Workflow
### 1. Configuration (prompts.yaml)
Define your entire AI infrastructure in YAML. **forthos** handles the validation of schema names, model tiers, and embedding dimensions at load time.
```yaml
path: "agent_orchestrator"
prompts:
  embeddings:
    - model: text-embedding-3-small
      input: "Knowledge base entry for Rust systems."
      dimensions: 512

  responses:
    # Example 1: Simple Text with Prompt Caching
    - model: gpt-5.4-mini
      service_tier: flex
      input: "Explain the ownership model in Rust."
      cache_retention: 24h
      text:
        verbosity: low
        format: { type: text }

    # Example 2: Structured Output with JSON Schema
    - model: gpt-5.4-mini
      input:
        - role: user
          content: "Extract: Alice is 25 and Bob is 30."
          type: message
      text:
        verbosity: high
        format:
          type: json_schema
          name: user_parser
          strict: true
          schema:
            type: object
            properties:
              users:
                type: array
                items:
                  type: object
                  properties:
                    name: { type: string }
                    age: { type: integer }
            required: ["users"]

```
### 2. Implementation
Your Rust code stays clean. Just load the config and call the prompts by their index.
```rust
use forthos::OpenAIClient;
use forthos::client::InferenceConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Load and validate everything in one go
  let config = InferenceConfig::from_yaml("prompts.yaml")?;
  let client = OpenAIClient::new("your-key")?.with_config(config);

  // 1. Run Embedding (Index 0)
  let emb = client.run_embedding_at(0).await?;
  println!("Vector generated: {} dims", emb.data[0].embedding.len());

  // 2. Run Structured Response (Index 1)
  let resp = client.run_response_at(1).await?;
  
  #[derive(forthos::Deserialize, Debug)]
  struct UserList { users: Vec<User> }
  #[derive(forthos::Deserialize, Debug)]
  struct User { name: String, age: u32 }

  let data = resp.parse_json::<UserList>()??;
  println!("Parsed {} users from AI response.", data.users.len());

  Ok(())
}

```
## 💎 Features
 * **Logic Separation**: Prompts, system instructions, and schemas live in YAML, not nested in your Rust strings.
 * **Validation Gatekeeper**: Catch common OpenAI API errors (like spaces in schema names or invalid model dimensions) during InferenceConfig::from_yaml.
 * **Performance**: Utilizes reqwest-middleware for retry strategies and optimized connection pooling.
## 📄 License
Licensed under the **MIT License**.
