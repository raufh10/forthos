pub mod embeddings;
pub mod responses;

pub use embeddings::{
  EmbeddingInput,
  EmbeddingModel,
  EmbeddingRequest,
  EncodingFormat
};

pub use responses::{
  CacheRetention,
  EasyInputMessage,
  Format,
  ResponseInput,
  ResponseModel,
  ResponseRequest,
  Role,
  Text,
  Verbosity,
};

pub fn create_message(
  role: Role, 
  content: &str, 
  phase: Option<&str>, 
  msg_type: Option<&str>
) -> EasyInputMessage {
  EasyInputMessage {
    role,
    content: content.to_string(),
    phase: phase.map(|p| p.to_string()),
    r#type: msg_type.unwrap_or("message").to_string(),
  }
}

