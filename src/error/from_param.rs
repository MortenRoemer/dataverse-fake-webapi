use std::{fmt::Display, error::Error};

#[derive(Clone, Debug)]
pub struct FromParamError {
    pub message: &'static str,
    pub found: String,
    pub target_type: &'static str,
    pub example: &'static str,
}

impl FromParamError {
    pub fn pattern_mismatch(found: String, target_type: &'static str, example: &'static str) -> Self {
        Self {
            message: "String was not matching the expected pattern",
            found,
            target_type,
            example
        }
    }
}

impl Display for FromParamError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter, 
            "Failed to parse '{}' into {}. Expected pattern: '{}'. Reason: {}", 
            self.found, 
            self.target_type, 
            self.example, 
            self.message
        ) 
    }
}

impl Error for FromParamError {}
