use crate::domain::clip::ClipError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Content(String);

impl Content {
    // return Result type to deal with fail when structure instantiation
    pub fn new(content: &str) -> Result<Self, ClipError> {
        if !content.trim().is_empty() {
            Ok(Self(content.to_owned()))
        } else {
            Err(ClipError::EmptyContent)
        }
    }

    // useful when convert the data to be suitable for database
    pub fn into_inner(self) -> String {
        self.0
    }

    // useful when logging the data, the content will still exist
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
