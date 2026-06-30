#![forbid(unsafe_code)]

pub type PolicyContractValidationResult = Result<(), &'static str>;

pub mod action;
pub mod app_game;
pub mod authority;
pub mod preview;
pub mod schedule;
pub mod screen_ai;
