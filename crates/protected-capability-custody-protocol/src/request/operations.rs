use crate::target::{Action, TargetDescriptor};

use super::{Request, RequestKind};

impl Request {
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

    pub fn opaque_token(&self) -> &[u8] {
        &self.opaque_token
    }
}
