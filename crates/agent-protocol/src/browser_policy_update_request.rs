use crate::browser_policy::{BrowserPolicyUpdateKind, BrowserPolicyUpdateRequest};

impl BrowserPolicyUpdateRequest {
    pub fn request_id(&self) -> &str {
        match self {
            Self::Get(request) => &request.request_id,
            Self::Preview(request) => &request.request_id,
            Self::Patch(request) => &request.request_id,
            Self::Replace(request) => &request.request_id,
            Self::Rollback(request) => &request.request_id,
        }
    }

    pub fn kind(&self) -> BrowserPolicyUpdateKind {
        match self {
            Self::Get(_) => BrowserPolicyUpdateKind::Get,
            Self::Preview(_) => BrowserPolicyUpdateKind::Preview,
            Self::Patch(_) => BrowserPolicyUpdateKind::Patch,
            Self::Replace(_) => BrowserPolicyUpdateKind::Replace,
            Self::Rollback(_) => BrowserPolicyUpdateKind::Rollback,
        }
    }
}
