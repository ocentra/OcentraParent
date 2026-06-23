use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[cfg(test)]
use std::sync::Mutex;

use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyRejectionReason;
use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateKind;
use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateRequest;
use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateResponse;
use ocentra_parent_agent_protocol::browser_policy_model::BrowserPolicyValue;
use ocentra_parent_agent_protocol::constants;
use tokio::sync::Mutex as AsyncMutex;

use crate::{
    browser_policy_compiler::compile_browser_policy,
    browser_policy_request::apply_browser_policy_patches,
    browser_policy_runtime_support::{
        accepted_response, base_revision_matches, default_policy, default_revision_id,
        next_audit_event_id, next_revision_id, preview_revision_id, rejected_response,
    },
    browser_policy_store::{
        browser_policy_store_path_from_env, read_browser_policy_state, write_browser_policy_state,
        BrowserPolicyAuditRecord, BrowserPolicyRevisionRecord, BrowserPolicyStoreError,
        BrowserPolicyStoredState,
    },
    time::timestamp_now,
};

#[derive(Clone, Debug)]
pub(crate) struct BrowserPolicyRuntime {
    persistence: BrowserPolicyPersistence,
    io_lock: Arc<AsyncMutex<()>>,
}

#[derive(Clone, Debug)]
enum BrowserPolicyPersistence {
    #[cfg(test)]
    InMemory(Arc<Mutex<BrowserPolicyStoredState>>),
    LocalJson(PathBuf),
}

impl BrowserPolicyRuntime {
    pub(crate) fn from_env() -> Self {
        Self::for_store_path(browser_policy_store_path_from_env())
    }

    #[cfg(test)]
    pub(crate) fn in_memory() -> Self {
        Self {
            persistence: BrowserPolicyPersistence::InMemory(Arc::new(Mutex::new(
                BrowserPolicyStoredState::empty(),
            ))),
            io_lock: Arc::new(AsyncMutex::new(())),
        }
    }

    pub(crate) fn for_store_path(path: impl AsRef<Path>) -> Self {
        Self {
            persistence: BrowserPolicyPersistence::LocalJson(path.as_ref().to_path_buf()),
            io_lock: Arc::new(AsyncMutex::new(())),
        }
    }

    pub(crate) async fn handle_request(
        &self,
        request: BrowserPolicyUpdateRequest,
    ) -> BrowserPolicyUpdateResponse {
        let _guard = self.io_lock.lock().await;
        match request {
            BrowserPolicyUpdateRequest::Get(request) => {
                self.handle_get(request.request_id, request.policy_id).await
            }
            BrowserPolicyUpdateRequest::Preview(request) => {
                self.handle_preview(request.request_id, request.policy)
            }
            BrowserPolicyUpdateRequest::Patch(request) => {
                self.handle_patch(
                    request.request_id,
                    request.policy_id,
                    request.base_revision_id,
                    request.patches,
                )
                .await
            }
            BrowserPolicyUpdateRequest::Replace(request) => {
                self.handle_replace(request.request_id, request.base_revision_id, request.policy)
                    .await
            }
            BrowserPolicyUpdateRequest::Rollback(request) => {
                self.handle_rollback(
                    request.request_id,
                    request.policy_id,
                    request.target_revision_id,
                )
                .await
            }
        }
    }

    async fn handle_get(
        &self,
        request_id: String,
        policy_id: String,
    ) -> BrowserPolicyUpdateResponse {
        let generated_at = timestamp_now();
        match self.read_state().await {
            Ok(state) => match state.active_revision() {
                Some(revision) if revision.policy.policy_id == policy_id => accepted_response(
                    request_id,
                    BrowserPolicyUpdateKind::Get,
                    revision.policy.clone(),
                    revision.effective_policy.clone(),
                    None,
                    constants::browser_policy::MESSAGE_REPORTED,
                    &generated_at,
                ),
                _ => {
                    let revision_id = default_revision_id();
                    let policy = default_policy(policy_id);
                    match compile_browser_policy(&policy, &revision_id, &generated_at) {
                        Ok(effective_policy) => accepted_response(
                            request_id,
                            BrowserPolicyUpdateKind::Get,
                            policy,
                            effective_policy,
                            None,
                            constants::browser_policy::MESSAGE_REPORTED,
                            &generated_at,
                        ),
                        Err(reason) => rejected_response(
                            request_id,
                            BrowserPolicyUpdateKind::Get,
                            reason,
                            constants::browser_policy::MESSAGE_INVALID_POLICY,
                            &generated_at,
                        ),
                    }
                }
            },
            Err(_) => rejected_response(
                request_id,
                BrowserPolicyUpdateKind::Get,
                BrowserPolicyRejectionReason::StorageUnavailable,
                constants::browser_policy::MESSAGE_STORAGE_UNAVAILABLE,
                &generated_at,
            ),
        }
    }

    fn handle_preview(
        &self,
        request_id: String,
        policy: BrowserPolicyValue,
    ) -> BrowserPolicyUpdateResponse {
        let generated_at = timestamp_now();
        let revision_id = preview_revision_id();
        match compile_browser_policy(&policy, &revision_id, &generated_at) {
            Ok(effective_policy) => accepted_response(
                request_id,
                BrowserPolicyUpdateKind::Preview,
                policy,
                effective_policy,
                None,
                constants::browser_policy::MESSAGE_PREVIEWED,
                &generated_at,
            ),
            Err(reason) => rejected_response(
                request_id,
                BrowserPolicyUpdateKind::Preview,
                reason,
                constants::browser_policy::MESSAGE_INVALID_POLICY,
                &generated_at,
            ),
        }
    }

    async fn handle_replace(
        &self,
        request_id: String,
        base_revision_id: Option<String>,
        policy: BrowserPolicyValue,
    ) -> BrowserPolicyUpdateResponse {
        let generated_at = timestamp_now();
        let state = match self.read_state().await {
            Ok(state) => state,
            Err(_) => {
                return rejected_response(
                    request_id,
                    BrowserPolicyUpdateKind::Replace,
                    BrowserPolicyRejectionReason::StorageUnavailable,
                    constants::browser_policy::MESSAGE_STORAGE_UNAVAILABLE,
                    &generated_at,
                )
            }
        };
        if let Err(reason) = base_revision_matches(&state, base_revision_id.as_deref()) {
            return rejected_response(
                request_id,
                BrowserPolicyUpdateKind::Replace,
                reason,
                constants::browser_policy::MESSAGE_STALE_REVISION,
                &generated_at,
            );
        }
        self.persist_revision(
            state,
            request_id,
            BrowserPolicyUpdateKind::Replace,
            policy,
            &generated_at,
        )
        .await
    }

    async fn handle_patch(
        &self,
        request_id: String,
        policy_id: String,
        base_revision_id: String,
        patches: Vec<ocentra_parent_agent_protocol::browser_policy::BrowserPolicyPatch>,
    ) -> BrowserPolicyUpdateResponse {
        let generated_at = timestamp_now();
        let state = match self.read_state().await {
            Ok(state) => state,
            Err(_) => {
                return rejected_response(
                    request_id,
                    BrowserPolicyUpdateKind::Patch,
                    BrowserPolicyRejectionReason::StorageUnavailable,
                    constants::browser_policy::MESSAGE_STORAGE_UNAVAILABLE,
                    &generated_at,
                )
            }
        };
        let Some(active) = state.active_revision() else {
            return rejected_response(
                request_id,
                BrowserPolicyUpdateKind::Patch,
                BrowserPolicyRejectionReason::RevisionNotFound,
                constants::browser_policy::MESSAGE_REVISION_NOT_FOUND,
                &generated_at,
            );
        };
        if active.revision_id != base_revision_id || active.policy.policy_id != policy_id {
            return rejected_response(
                request_id,
                BrowserPolicyUpdateKind::Patch,
                BrowserPolicyRejectionReason::StaleRevision,
                constants::browser_policy::MESSAGE_STALE_REVISION,
                &generated_at,
            );
        }
        let policy = match apply_browser_policy_patches(active.policy.clone(), &patches) {
            Ok(policy) => policy,
            Err(reason) => {
                return rejected_response(
                    request_id,
                    BrowserPolicyUpdateKind::Patch,
                    reason,
                    constants::browser_policy::MESSAGE_INVALID_REQUEST,
                    &generated_at,
                )
            }
        };
        self.persist_revision(
            state,
            request_id,
            BrowserPolicyUpdateKind::Patch,
            policy,
            &generated_at,
        )
        .await
    }

    async fn handle_rollback(
        &self,
        request_id: String,
        policy_id: String,
        target_revision_id: String,
    ) -> BrowserPolicyUpdateResponse {
        let generated_at = timestamp_now();
        let mut state = match self.read_state().await {
            Ok(state) => state,
            Err(_) => {
                return rejected_response(
                    request_id,
                    BrowserPolicyUpdateKind::Rollback,
                    BrowserPolicyRejectionReason::StorageUnavailable,
                    constants::browser_policy::MESSAGE_STORAGE_UNAVAILABLE,
                    &generated_at,
                )
            }
        };
        let Some(target) = state.revision_by_id(&target_revision_id).cloned() else {
            return rejected_response(
                request_id,
                BrowserPolicyUpdateKind::Rollback,
                BrowserPolicyRejectionReason::RevisionNotFound,
                constants::browser_policy::MESSAGE_REVISION_NOT_FOUND,
                &generated_at,
            );
        };
        if target.policy.policy_id != policy_id {
            return rejected_response(
                request_id,
                BrowserPolicyUpdateKind::Rollback,
                BrowserPolicyRejectionReason::RevisionNotFound,
                constants::browser_policy::MESSAGE_REVISION_NOT_FOUND,
                &generated_at,
            );
        }
        let audit_event_id = next_audit_event_id(&state);
        state.active_revision_id = Some(target.revision_id.clone());
        state.audit_events.push(BrowserPolicyAuditRecord {
            audit_event_id: audit_event_id.clone(),
            request_id: request_id.clone(),
            kind: BrowserPolicyUpdateKind::Rollback,
            revision_id: target.revision_id.clone(),
            created_at: generated_at.clone(),
        });
        if self.write_state(&state).await.is_err() {
            return rejected_response(
                request_id,
                BrowserPolicyUpdateKind::Rollback,
                BrowserPolicyRejectionReason::StorageUnavailable,
                constants::browser_policy::MESSAGE_STORAGE_UNAVAILABLE,
                &generated_at,
            );
        }
        accepted_response(
            request_id,
            BrowserPolicyUpdateKind::Rollback,
            target.policy,
            target.effective_policy,
            Some(audit_event_id),
            constants::browser_policy::MESSAGE_ROLLBACK_ACCEPTED,
            &generated_at,
        )
    }

    async fn persist_revision(
        &self,
        mut state: BrowserPolicyStoredState,
        request_id: String,
        kind: BrowserPolicyUpdateKind,
        policy: BrowserPolicyValue,
        generated_at: &str,
    ) -> BrowserPolicyUpdateResponse {
        let revision_id = next_revision_id(&state);
        let audit_event_id = next_audit_event_id(&state);
        let effective_policy = match compile_browser_policy(&policy, &revision_id, generated_at) {
            Ok(effective_policy) => effective_policy,
            Err(reason) => {
                return rejected_response(
                    request_id,
                    kind,
                    reason,
                    constants::browser_policy::MESSAGE_INVALID_POLICY,
                    generated_at,
                )
            }
        };
        state.active_revision_id = Some(revision_id.clone());
        state.revisions.push(BrowserPolicyRevisionRecord {
            revision_id: revision_id.clone(),
            policy: policy.clone(),
            effective_policy: effective_policy.clone(),
            created_at: generated_at.to_string(),
            audit_event_id: audit_event_id.clone(),
        });
        state.audit_events.push(BrowserPolicyAuditRecord {
            audit_event_id: audit_event_id.clone(),
            request_id: request_id.clone(),
            kind,
            revision_id,
            created_at: generated_at.to_string(),
        });
        if self.write_state(&state).await.is_err() {
            return rejected_response(
                request_id,
                kind,
                BrowserPolicyRejectionReason::StorageUnavailable,
                constants::browser_policy::MESSAGE_STORAGE_UNAVAILABLE,
                generated_at,
            );
        }
        accepted_response(
            request_id,
            kind,
            policy,
            effective_policy,
            Some(audit_event_id),
            constants::browser_policy::MESSAGE_ACCEPTED,
            generated_at,
        )
    }

    async fn read_state(&self) -> Result<BrowserPolicyStoredState, BrowserPolicyStoreError> {
        match &self.persistence {
            #[cfg(test)]
            BrowserPolicyPersistence::InMemory(state) => {
                state.lock().map(|state| state.clone()).map_err(|error| {
                    let _ = error;
                    BrowserPolicyStoreError::Unavailable
                })
            }
            BrowserPolicyPersistence::LocalJson(path) => read_browser_policy_state(path).await,
        }
    }

    async fn write_state(
        &self,
        state: &BrowserPolicyStoredState,
    ) -> Result<(), BrowserPolicyStoreError> {
        match &self.persistence {
            #[cfg(test)]
            BrowserPolicyPersistence::InMemory(current) => current
                .lock()
                .map(|mut current| {
                    *current = state.clone();
                })
                .map_err(|error| {
                    let _ = error;
                    BrowserPolicyStoreError::Unavailable
                }),
            BrowserPolicyPersistence::LocalJson(path) => {
                write_browser_policy_state(path, state).await
            }
        }
    }
}
