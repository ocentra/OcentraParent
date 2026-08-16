use std::fmt;

use super::ChildAgentIngressError;

impl fmt::Display for ChildAgentIngressError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueueFull => formatter.write_str("child service command queue is full"),
            Self::ServiceClosed => formatter.write_str("child service command queue is closed"),
            Self::Service(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ChildAgentIngressError {}
