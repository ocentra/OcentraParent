use super::AiWorkRequest;
use crate::ai_contracts::context::{AiOwnerResolvedRuntime, AiPromptReference};
use crate::ai_contracts::identity::{AiRequestId, AiSchemaIdentity, AiTimestamp, AiWorkItemId};

impl AiWorkRequest {
    pub fn new(
        identity: AiSchemaIdentity,
        work_item_id: AiWorkItemId,
        work_kind: super::AiWorkKind,
        requested_at: AiTimestamp,
        deadline_at: Option<AiTimestamp>,
        retry_policy: super::AiRetryPolicy,
        prompt: Option<AiPromptReference>,
    ) -> Result<Self, &'static str> {
        Self::new_with_owner_runtime(
            identity,
            work_item_id,
            work_kind,
            requested_at,
            deadline_at,
            retry_policy,
            prompt,
            AiOwnerResolvedRuntime::absent(),
        )
    }

    pub(crate) fn new_with_owner_runtime(
        identity: AiSchemaIdentity,
        work_item_id: AiWorkItemId,
        work_kind: super::AiWorkKind,
        requested_at: AiTimestamp,
        deadline_at: Option<AiTimestamp>,
        retry_policy: super::AiRetryPolicy,
        prompt: Option<AiPromptReference>,
        runtime: AiOwnerResolvedRuntime,
    ) -> Result<Self, &'static str> {
        if !requested_at.is_well_formed()
            || deadline_at
                .as_ref()
                .is_some_and(|deadline| !requested_at.precedes(deadline))
        {
            return Err("AI work request has an invalid requested/deadline timestamp");
        }
        Ok(Self {
            identity,
            work_item_id,
            work_kind,
            requested_at,
            deadline_at,
            retry_policy,
            prompt,
            runtime: runtime.into_runtime(),
        })
    }

    pub fn identity(&self) -> &AiSchemaIdentity {
        &self.identity
    }

    pub fn work_item_id(&self) -> &AiWorkItemId {
        &self.work_item_id
    }

    pub fn work_kind(&self) -> super::AiWorkKind {
        self.work_kind
    }

    pub fn request_id(&self) -> &AiRequestId {
        self.identity.request_id()
    }

    pub(super) fn requested_at(&self) -> &AiTimestamp {
        &self.requested_at
    }
}
