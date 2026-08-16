use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use chrono::Utc;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::transport::{
    PolicyRequestAssistantPreviewConfirmAction, PolicyRequestAssistantPreviewConfirmActorRole,
    PolicyRequestAssistantPreviewConfirmActorState, PolicyRequestAssistantPreviewConfirmRequest,
    PolicyRequestAssistantPreviewConfirmRequestKind,
    PolicyRequestAssistantPreviewConfirmTargetKind,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_schema::parent_ui_bridge::{
    ParentPolicyPreviewId, ParentPolicyPreviewReadModelSnapshot, ParentPortalParentAccessState,
};
use serde::Deserialize;
use serde_json::Value;

const DRAFT_PAYLOAD_FIELD: &str = "policyPreviewAuthoringDraft";
const HANDLE_PAYLOAD_FIELD: &str = "policyPreviewAuthoringHandle";
const MAX_STAGED_DRAFTS: usize = 32;
const STAGED_DRAFT_TTL: Duration = Duration::from_secs(300);

#[derive(Clone, Debug)]
pub(super) struct StagedPolicyPreviewDraft {
    pub(super) handle: String,
    pub(super) read_model: ParentPolicyPreviewReadModelSnapshot,
    pub(super) target_value: String,
    pub(super) requested_action: String,
}

#[derive(Clone, Debug)]
struct StoredPolicyPreviewDraft {
    preview_id: String,
    read_model: ParentPolicyPreviewReadModelSnapshot,
    target_value: String,
    requested_action: String,
    parent_access_state: ParentPortalParentAccessState,
    issued_at: Instant,
    in_flight: bool,
}

#[derive(Default)]
struct PolicyPreviewAuthoringStore {
    entries: HashMap<String, StoredPolicyPreviewDraft>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PolicyPreviewAuthoringDraftInput {
    target_value: String,
    requested_action: String,
}

fn store() -> &'static Mutex<PolicyPreviewAuthoringStore> {
    static STORE: OnceLock<Mutex<PolicyPreviewAuthoringStore>> = OnceLock::new();
    STORE.get_or_init(|| Mutex::new(PolicyPreviewAuthoringStore::default()))
}

pub(super) fn stage(
    payload: &Value,
    read_model: &ParentPolicyPreviewReadModelSnapshot,
    parent_access_state: &ParentPortalParentAccessState,
) -> Result<String, String> {
    let preview_id = read_model
        .preview_id
        .as_ref()
        .ok_or_else(|| "policy preview identifier is missing".to_string())?;
    let draft_text = exact_payload_field(payload, DRAFT_PAYLOAD_FIELD, "draft")
        .and_then(Value::as_str)
        .ok_or_else(|| "policy preview draft payload is missing".to_string())?;
    let draft: PolicyPreviewAuthoringDraftInput = serde_json::from_str(draft_text)
        .map_err(|_| "policy preview draft payload is invalid".to_string())?;
    let target_value = draft.target_value.trim();
    if target_value.is_empty() || target_value.len() > 256 {
        return Err("policy preview draft target is invalid".to_string());
    }
    let requested_action = draft.requested_action.trim().to_ascii_lowercase();
    if !matches!(
        requested_action.as_str(),
        "allow" | "warn" | "ask-parent" | "time-limit" | "block"
    ) {
        return Err("policy preview draft action is invalid".to_string());
    }

    let mut authoring_store = store()
        .lock()
        .map_err(|_| "policy preview authoring store is unavailable".to_string())?;
    let now = Instant::now();
    authoring_store
        .entries
        .retain(|_, entry| now.duration_since(entry.issued_at) <= STAGED_DRAFT_TTL);
    if authoring_store.entries.values().any(|entry| {
        entry.preview_id == preview_id.as_str()
            && entry.parent_access_state == *parent_access_state
            && entry.in_flight
    }) {
        return Err("policy preview confirmation is already pending".to_string());
    }
    authoring_store.entries.retain(|_, entry| {
        !(entry.preview_id == preview_id.as_str()
            && entry.parent_access_state == *parent_access_state)
    });
    while authoring_store.entries.len() >= MAX_STAGED_DRAFTS {
        let Some(oldest_handle) = authoring_store
            .entries
            .iter()
            .min_by_key(|(_, entry)| entry.issued_at)
            .map(|(handle, _)| handle.clone())
        else {
            break;
        };
        authoring_store.entries.remove(&oldest_handle);
    }

    let handle = opaque_handle(&authoring_store)?;
    authoring_store.entries.insert(
        handle.clone(),
        StoredPolicyPreviewDraft {
            preview_id: preview_id.as_str().to_string(),
            read_model: read_model.clone(),
            target_value: target_value.to_string(),
            requested_action,
            parent_access_state: parent_access_state.clone(),
            issued_at: now,
            in_flight: false,
        },
    );
    Ok(handle)
}

pub(super) fn current(
    read_model: &ParentPolicyPreviewReadModelSnapshot,
    parent_access_state: &ParentPortalParentAccessState,
) -> Option<StagedPolicyPreviewDraft> {
    let preview_id = read_model.preview_id.as_ref()?.as_str();
    let authoring_store = store().lock().ok()?;
    authoring_store
        .entries
        .iter()
        .find(|(_, entry)| {
            entry.preview_id == preview_id
                && entry.parent_access_state == *parent_access_state
                && !entry.in_flight
                && Instant::now().duration_since(entry.issued_at) <= STAGED_DRAFT_TTL
        })
        .map(|(handle, entry)| StagedPolicyPreviewDraft {
            handle: handle.clone(),
            read_model: entry.read_model.clone(),
            target_value: entry.target_value.clone(),
            requested_action: entry.requested_action.clone(),
        })
}

pub(super) fn handle_payload(handle: &str) -> Value {
    serde_json::json!({ HANDLE_PAYLOAD_FIELD: handle })
}

pub(super) fn consume(
    payload: &Value,
    preview_id: &ParentPolicyPreviewId,
    parent_access_state: &ParentPortalParentAccessState,
) -> Result<StagedPolicyPreviewDraft, String> {
    let handle = exact_payload_field(payload, HANDLE_PAYLOAD_FIELD, "handle")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "policy preview authoring handle is missing".to_string())?;
    let mut authoring_store = store()
        .lock()
        .map_err(|_| "policy preview authoring store is unavailable".to_string())?;
    let Some(entry) = authoring_store.entries.get(handle) else {
        return Err("policy preview authoring handle is unknown or already consumed".to_string());
    };
    if entry.preview_id != preview_id.as_str() {
        return Err("policy preview authoring handle is bound to another preview".to_string());
    }
    if entry.parent_access_state != *parent_access_state {
        return Err(
            "policy preview authoring handle role does not match current authority".to_string(),
        );
    }
    if !matches!(
        parent_access_state,
        ParentPortalParentAccessState::ActiveController
    ) {
        return Err(
            "policy preview authoring confirmation requires active controller authority"
                .to_string(),
        );
    }
    if Instant::now().duration_since(entry.issued_at) > STAGED_DRAFT_TTL {
        authoring_store.entries.remove(handle);
        return Err("policy preview authoring handle is stale".to_string());
    }
    if entry.in_flight {
        return Err("policy preview confirmation is already pending".to_string());
    }
    let entry = authoring_store.entries.get_mut(handle).ok_or_else(|| {
        "policy preview authoring handle is unknown or already consumed".to_string()
    })?;
    entry.in_flight = true;
    Ok(StagedPolicyPreviewDraft {
        handle: handle.to_string(),
        read_model: entry.read_model.clone(),
        target_value: entry.target_value.clone(),
        requested_action: entry.requested_action.clone(),
    })
}

pub(super) fn commit(
    draft: &StagedPolicyPreviewDraft,
    preview_id: &ParentPolicyPreviewId,
    parent_access_state: &ParentPortalParentAccessState,
) -> Result<(), String> {
    let mut authoring_store = store()
        .lock()
        .map_err(|_| "policy preview authoring store is unavailable".to_string())?;
    let Some(entry) = authoring_store.entries.get(&draft.handle) else {
        return Err("policy preview authoring handle is unknown or already consumed".to_string());
    };
    if entry.preview_id != preview_id.as_str() {
        return Err("policy preview authoring handle is bound to another preview".to_string());
    }
    if entry.parent_access_state != *parent_access_state {
        return Err(
            "policy preview authoring handle role does not match current authority".to_string(),
        );
    }
    if !entry.in_flight {
        return Err("policy preview authoring handle is not pending confirmation".to_string());
    }
    authoring_store.entries.remove(&draft.handle);
    Ok(())
}

pub(super) fn release(
    draft: &StagedPolicyPreviewDraft,
    preview_id: &ParentPolicyPreviewId,
    parent_access_state: &ParentPortalParentAccessState,
) -> Result<(), String> {
    let mut authoring_store = store()
        .lock()
        .map_err(|_| "policy preview authoring store is unavailable".to_string())?;
    let Some(entry) = authoring_store.entries.get_mut(&draft.handle) else {
        return Err("policy preview authoring handle is unknown or already consumed".to_string());
    };
    if entry.preview_id != preview_id.as_str() {
        return Err("policy preview authoring handle is bound to another preview".to_string());
    }
    if entry.parent_access_state != *parent_access_state {
        return Err(
            "policy preview authoring handle role does not match current authority".to_string(),
        );
    }
    entry.in_flight = false;
    Ok(())
}

pub(super) fn typed_confirm_payload(draft: &StagedPolicyPreviewDraft) -> Result<Value, String> {
    let context = draft
        .read_model
        .confirmation_context
        .as_ref()
        .ok_or_else(|| {
            "policy preview confirmation context is unavailable; manual review required".to_string()
        })?;
    let target_kind = match draft.read_model.target_type.as_deref() {
        Some("app") => PolicyRequestAssistantPreviewConfirmTargetKind::App,
        Some("device") => PolicyRequestAssistantPreviewConfirmTargetKind::Device,
        Some("site") | Some("domain") => PolicyRequestAssistantPreviewConfirmTargetKind::Site,
        Some("category") => PolicyRequestAssistantPreviewConfirmTargetKind::Category,
        Some(value) => {
            return Err(format!(
                "policy preview target kind cannot be confirmed: {value}"
            ));
        }
        None => return Err("policy preview target kind is missing".to_string()),
    };
    let trusted_action = draft
        .read_model
        .network_requested_policy_action
        .as_deref()
        .or_else(|| {
            draft
                .read_model
                .decision_action
                .as_ref()
                .map(|value| value.as_str())
        })
        .ok_or_else(|| {
            "policy preview trusted requested action is unavailable; manual review required"
                .to_string()
        })?;
    if trusted_action != draft.requested_action {
        return Err("policy preview draft action does not match trusted request".to_string());
    }
    let requested_action = match trusted_action {
        "allow" => PolicyRequestAssistantPreviewConfirmAction::Allow,
        "warn" => PolicyRequestAssistantPreviewConfirmAction::Warn,
        "ask-parent" => PolicyRequestAssistantPreviewConfirmAction::AskParent,
        "time-limit" => PolicyRequestAssistantPreviewConfirmAction::TimeLimit,
        "block" => PolicyRequestAssistantPreviewConfirmAction::Block,
        value => {
            return Err(format!(
                "policy preview action cannot be confirmed: {value}"
            ))
        }
    };
    let preview_id = draft
        .read_model
        .preview_id
        .as_ref()
        .ok_or_else(|| "policy preview identifier is missing".to_string())?;
    if required_context(&context.assistant_preview_id, "assistant preview")? != preview_id.as_str()
    {
        return Err(
            "policy preview trusted assistant preview does not match current preview".to_string(),
        );
    }
    let target_reference_id = required_context(&context.target_reference_id, "target reference")?;
    if draft.target_value != target_reference_id {
        return Err("policy preview draft target does not match trusted request".to_string());
    }
    if draft
        .read_model
        .target_value
        .as_deref()
        .is_some_and(|value| value != target_reference_id)
    {
        return Err("policy preview trusted target does not match source request".to_string());
    }
    let actor_role = match required_context(&context.actor_role, "actor role")? {
        constants::policy_control::source::ROLE_PARENT => {
            PolicyRequestAssistantPreviewConfirmActorRole::Parent
        }
        constants::policy_control::source::ROLE_CO_PARENT => {
            PolicyRequestAssistantPreviewConfirmActorRole::CoParent
        }
        value => return Err(format!("policy preview actor role cannot confirm: {value}")),
    };
    let actor_state = match required_context(&context.actor_state, "actor state")? {
        constants::policy_control::source::ACTOR_STATE_ACTIVE => {
            PolicyRequestAssistantPreviewConfirmActorState::Active
        }
        constants::policy_control::source::ACTOR_STATE_REVOKED => {
            PolicyRequestAssistantPreviewConfirmActorState::Revoked
        }
        value => return Err(format!("policy preview actor state is invalid: {value}")),
    };
    let audit_reference_ids = required_context(&context.audit_reference_ids, "audit references")?
        .split(constants::delimiter::LIST)
        .filter(|value| !value.trim().is_empty())
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    if audit_reference_ids.is_empty() {
        return Err(
            "policy preview audit references are empty; manual review required".to_string(),
        );
    }
    let now = Utc::now();
    let request = PolicyRequestAssistantPreviewConfirmRequest {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: format!("policy-preview-confirm-{}", draft.handle),
        request_id: required_context(&context.request_id, "request id")?.to_string(),
        submission_key: required_context(&context.submission_key, "submission key")?.to_string(),
        household_id: required_context(&context.household_id, "household")?.to_string(),
        child_profile_id: required_context(&context.child_profile_id, "child profile")?.to_string(),
        device_id: Some(required_context(&context.device_id, "device")?.to_string()),
        source_document_id: required_context(&context.source_document_id, "source document")?.to_string(),
        policy_version: context
            .policy_version
            .ok_or_else(|| "policy preview policy version is unavailable; manual review required".to_string())?,
        request_kind: PolicyRequestAssistantPreviewConfirmRequestKind::AskParent,
        target_kind,
        target_reference_id: target_reference_id.to_string(),
        requested_action,
        rule_id: context.rule_id.clone(),
        requested_bonus_minutes: None,
        requested_at: required_context(&context.requested_at, "request timestamp")?.to_string(),
        expires_at: required_context(&context.expires_at, "request expiry")?.to_string(),
        origin: ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestOrigin::AssistantDraft,
        assistant_preview_id: preview_id.as_str().to_string(),
        assistant_confirmation_state:
            ocentra_parent_agent_protocol::activity::policy_preview::PolicyAssistantConfirmationState::ParentConfirmationRequired,
        request_status: ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus::PreviewOnly,
        audit_reference_ids,
        confirmation_actor_id: required_context(&context.actor_id, "actor id")?.to_string(),
        confirmation_actor_role: actor_role,
        confirmation_actor_state: actor_state,
        confirmation_audit_reference_id: required_context(
            &context.confirmation_audit_reference_id,
            "confirmation audit reference",
        )?
        .to_string(),
        confirmed_at: now.to_rfc3339(),
    };
    let request_text = serde_json::to_string(&request)
        .map_err(|_| "policy preview confirmation request could not be serialized".to_string())?;
    Ok(serde_json::json!({
        constants::field::POLICY_REQUEST_ASSISTANT_PREVIEW_CONFIRM_REQUEST: request_text,
    }))
}

fn required_context<'a>(value: &'a Option<String>, label: &str) -> Result<&'a str, String> {
    value
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| format!("policy preview {label} is unavailable; manual review required"))
}

pub(super) fn cancel(
    payload: &Value,
    preview_id: &ParentPolicyPreviewId,
    parent_access_state: &ParentPortalParentAccessState,
) -> Result<(), String> {
    if payload.as_object().is_some_and(|object| object.is_empty()) {
        return Ok(());
    }
    let handle = exact_payload_field(payload, HANDLE_PAYLOAD_FIELD, "handle")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "policy preview authoring handle is missing".to_string())?;
    let mut authoring_store = store()
        .lock()
        .map_err(|_| "policy preview authoring store is unavailable".to_string())?;
    let Some(entry) = authoring_store.entries.get(handle) else {
        return Err("policy preview authoring handle is unknown or already consumed".to_string());
    };
    if entry.preview_id != preview_id.as_str() {
        return Err("policy preview authoring handle is bound to another preview".to_string());
    }
    if entry.parent_access_state != *parent_access_state {
        return Err(
            "policy preview authoring handle role does not match current authority".to_string(),
        );
    }
    if entry.in_flight {
        return Err("policy preview confirmation is already pending".to_string());
    }
    authoring_store.entries.remove(handle);
    Ok(())
}

fn opaque_handle(authoring_store: &PolicyPreviewAuthoringStore) -> Result<String, String> {
    for _ in 0..4 {
        let mut random = [0_u8; 24];
        getrandom::fill(&mut random)
            .map_err(|_| "policy preview authoring handle entropy is unavailable".to_string())?;
        let handle = format!(
            "ppah-{}",
            random
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>()
        );
        if !authoring_store.entries.contains_key(&handle) {
            return Ok(handle);
        }
    }
    Err("policy preview authoring handle generation collided".to_string())
}

fn exact_payload_field<'a>(
    payload: &'a Value,
    field: &str,
    label: &str,
) -> Result<&'a Value, String> {
    let object = payload
        .as_object()
        .ok_or_else(|| format!("policy preview {label} payload must be an object"))?;
    if object.len() != 1 || !object.contains_key(field) {
        return Err(format!(
            "policy preview {label} payload must contain only the expected field"
        ));
    }
    object
        .get(field)
        .ok_or_else(|| format!("policy preview {label} payload is missing"))
}
