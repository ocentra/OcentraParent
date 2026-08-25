use crate::target::{Action, TargetDescriptor};

use super::super::{ExpectedGenerations, RequestKind, UntrustedRequest};

impl UntrustedRequest {
    pub fn expected_generations(&self) -> ExpectedGenerations {
        self.expected_generations
    }

    pub fn kind(&self) -> RequestKind {
        self.kind
    }

    pub fn operation(&self) -> &[u8] {
        &self.operation
    }

    pub fn action(&self) -> Action {
        self.action
    }

    pub fn target(&self) -> &TargetDescriptor {
        &self.target
    }
}
