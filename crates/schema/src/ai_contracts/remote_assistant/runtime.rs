use super::AiRemoteAssistantOwnerResolvedRuntime;
use crate::ai_contracts::context::AiRuntimeReference;

impl AiRemoteAssistantOwnerResolvedRuntime {
    pub(crate) fn from_owner(runtime: Option<AiRuntimeReference>) -> Self {
        Self { runtime }
    }

    pub(super) fn into_runtime(self) -> Option<AiRuntimeReference> {
        self.runtime
    }
}
