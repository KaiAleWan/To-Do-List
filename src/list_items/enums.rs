
/// The `Priority` enum is used to store the priority assigned to an Item in the ToDoList.
#[derive(Debug, Clone)]
pub enum Priority {
    /// Indicates low priority task
    Low,
    /// Indicates medium priority task
    Medium,
    /// Indicates high priority task
    High,
    /// Is assigned when an invalid value was submitted to initialize the Priority struct
    Invalid,
}

impl Priority {
    /// Derives a new Priority instance from a &str input value. 
    /// Permissable values are "low", "medium", or "high". For any other value, the `invalid` Priority is assigned.
    /// 
    /// # Arguments
    /// * item_name : &str - Desired Priority variant
    /// 
    /// # Returns
    /// * `Priority`: A new Priority instance
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

/// Enum to handle errors caused by the invalid selection of ToDOList Items.
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