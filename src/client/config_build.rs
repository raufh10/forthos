use std::fs;
use std::path::Path;
use crate::client::models::InferenceConfig;
use crate::responses::Format;

impl InferenceConfig {
  pub fn from_yaml<P: AsRef<Path>>(path: P) -> Result<Self, String> {
    let content = fs::read_to_string(&path)
      .map_err(|e| format!("Failed to read config file: {}", e))?;

    let mut config: InferenceConfig = yaml_serde::from_str(&content)
      .map_err(|e| format!("YAML parsing error: {}", e))?;

    config.path = path.as_ref().to_string_lossy().to_string();
    config.validate_prompts()?;

    Ok(config)
  }

  fn validate_prompts(&self) -> Result<(), String> {
    for (i, resp) in self.prompts.responses.iter().enumerate() {
      if let Format::JsonSchema { schema, name, .. } = &resp.text.format {
        // 1. Ensure schema is an object
        if !schema.is_object() {
          return Err(format!("Response prompt #{} ({}) has an invalid schema: must be a JSON object", i, name));
        }

        // 2. Basic OpenAI Structured Output check: must have a 'type'
        if schema.get("type").is_none() {
          return Err(format!("Response prompt #{} ({}) schema is missing top-level 'type' field", i, name));
        }
        
        // 3. Name check (OpenAI restriction: alphanumeric, underscores, dashes, max 64)
        if name.chars().any(|c| !c.is_alphanumeric() && c != '_' && c != '-') {
          return Err(format!("Invalid schema name '{}': use only alphanumeric, underscores, or dashes", name));
        }
      }
    }
    Ok(())
  }
}
