#![forbid(unsafe_code)]

use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolicyContractValidationError(pub &'static str);

impl Display for PolicyContractValidationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.0)
    }
}

impl std::error::Error for PolicyContractValidationError {}

impl From<&'static str> for PolicyContractValidationError {
    fn from(value: &'static str) -> Self {
        Self(value)
    }
}

pub type PolicyContractValidationResult = Result<(), PolicyContractValidationError>;

pub mod action;
pub mod app_game;
pub mod authority;
pub mod preview;
pub mod schedule;
pub mod screen_ai;
