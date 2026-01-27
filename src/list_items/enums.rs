pub enum Priority {
    Low,
    Medium,
    High,
    Invalid,
}

impl Priority {
    pub fn from_str(input: &str) -> Self {
        if input.to_lowercase().eq("low") {
            Self::Low
        } else if input.to_lowercase().eq("medium") {
            Self::Medium
        } else if input.to_lowercase().eq("high") {
            Self::High
        } else {
            Self::Invalid
        }
    }
}

use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
#[non_exhaustive]
pub enum ToDoSelectionError {
    ToDoNotFound,
    ToDoAlreadyPresent,
}

impl Display for ToDoSelectionError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use ToDoSelectionError::*;
        match self {
            ToDoNotFound => write!(
                f,
                "The expected To-Do item does not exist."
            ),
            ToDoAlreadyPresent => write!(
                f,
                "The submitted To-Do item already exists."
            ),
        }
    }
}

impl Error for ToDoSelectionError {}