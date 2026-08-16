use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use chrono::Utc;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel;
use ocentra_parent_agent_protocol::transport::{
    PolicyRequestAssistantPreviewConfirmAction, PolicyRequestAssistantPreviewConfirmActorRole,
    PolicyRequestAssistantPreviewConfirmActorState, PolicyRequestParentResolutionDecision,
    PolicyRequestParentResolutionDeliveryBinding, PolicyRequestParentResolutionRequest,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_schema::parent_ui_bridge::{
    ParentPolicyPreviewReadModelSnapshot, ParentPortalParentAccessState,
};
use serde::Deserialize;
use serde_json::Value;

const RESOLUTION_PAYLOAD_FIELD: &str = constants::field::POLICY_REQUEST_PARENT_RESOLUTION_REQUEST;
const MAX_STAGED_RESOLUTIONS: usize = 32;
const STAGED_RESOLUTION_TTL: Duration = Duration::from_secs(300);

#[derive(Clone, Debug)]
pub(super) struct StagedParentResolution {
    pub(super) handle: String,
    pub(super) preview_id: String,
    pub(super) parent_actor_id: String,
    pub(super) request: PolicyRequestParentResolutionRequest,
}

#[derive(Clone, Debug)]
struct StoredParentResolution {
    preview_id: String,
    parent_actor_id: String,
    issued_at: Instant,
    in_flight: bool,
}

#[derive(Default)]
struct ParentResolutionStore {
    entries: HashMap<String, StoredParentResolution>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ParentResolutionDecisionInput {
    decision: PolicyRequestParentResolutionDecision,
}

fn store() -> &'static Mutex<ParentResolutionStore> {
    static STORE: OnceLock<Mutex<ParentResolutionStore>> = OnceLock::new();
    STORE.get_or_init(|| Mutex::new(ParentResolutionStore::default()))
}

pub(super) fn begin(
    payload: &Value,
    read_model: &ParentPolicyPreviewReadModelSnapshot,
    parent_access_state: &ParentPortalParentAccessState,
    lan_read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> Result<StagedParentResolution, String> {
    if !matches!(
        parent_access_state,
        ParentPortalParentAccessState::ActiveController
    ) {
        return Err("parent resolution requires active controller authority".to_string());
    }

    let decision = decision_from_payload(payload)?;
    let context = read_model.confirmation_context.as_ref().ok_or_else(|| {
        "parent resolution context is unavailable; manual review required".to_string()
    })?;
    let actor_id = required_context(&context.actor_id, "actor id")?;
    let local_actor_id = local_controller_actor_id(lan_read_model)?;
    if local_actor_id != actor_id {
        return Err(
            "parent resolution actor does not match local controller authority".to_string(),
        );
    }
    let actor_role = actor_role(&context.actor_role)?;
    if !matches!(
        actor_role,
        PolicyRequestAssistantPreviewConfirmActorRole::Parent
            | PolicyRequestAssistantPreviewConfirmActorRole::CoParent
    ) {
        return Err("parent resolution actor role is not approver-capable".to_string());
    }
    let actor_state = actor_state(&context.actor_state)?;
    if actor_state != PolicyRequestAssistantPreviewConfirmActorState::Active {
        return Err("parent resolution actor is not active".to_string());
    }
    let preview_id = read_model
        .preview_id
        .as_ref()
        .ok_or_else(|| "parent resolution preview identifier is missing".to_string())?;
    let approval_id = read_model
        .policy_approval_id
        .as_ref()
        .map(|value| value.as_str().to_string())
        .ok_or_else(|| {
            "parent resolution approval identifier is unavailable; manual review required"
                .to_string()
        })?;
    let confirmed_audit_reference_id = required_context(
        &context.confirmation_audit_reference_id,
        "confirmation audit reference",
    )?;
    let approval_audit_reference_id =
        single_audit_reference(context.audit_reference_ids.as_deref())?;

    let approved_action = match decision {
        PolicyRequestParentResolutionDecision::Grant
        | PolicyRequestParentResolutionDecision::Modify => Some(approved_action(read_model)?),
        PolicyRequestParentResolutionDecision::Deny
        | PolicyRequestParentResolutionDecision::Expire => None,
    };
    let delivery_binding = match decision {
        PolicyRequestParentResolutionDecision::Grant
        | PolicyRequestParentResolutionDecision::Modify => {
            Some(PolicyRequestParentResolutionDeliveryBinding {
                household_id: required_context(&context.household_id, "household")?.to_string(),
                child_profile_id: required_context(&context.child_profile_id, "child profile")?
                    .to_string(),
                device_id: context.device_id.clone(),
                source_document_id: required_context(
                    &context.source_document_id,
                    "source document",
                )?
                .to_string(),
                policy_version: context.policy_version.ok_or_else(|| {
                    "parent resolution policy version is unavailable; manual review required"
                        .to_string()
                })?,
            })
        }
        PolicyRequestParentResolutionDecision::Deny
        | PolicyRequestParentResolutionDecision::Expire => None,
    };

    let handle = opaque_handle()?;
    let request = PolicyRequestParentResolutionRequest {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: format!("policy-parent-resolution-{handle}"),
        confirmed_audit_reference_id: confirmed_audit_reference_id.to_string(),
        approval_id,
        parent_actor_id: actor_id.to_string(),
        parent_actor_role: actor_role,
        parent_actor_state: actor_state,
        decision,
        approved_action,
        approved_bonus_minutes: None,
        override_expires_at: match decision {
            PolicyRequestParentResolutionDecision::Grant
            | PolicyRequestParentResolutionDecision::Modify => {
                Some(required_context(&context.expires_at, "override expiry")?.to_string())
            }
            PolicyRequestParentResolutionDecision::Deny
            | PolicyRequestParentResolutionDecision::Expire => None,
        },
        decided_at: Utc::now().to_rfc3339(),
        approval_audit_reference_id: approval_audit_reference_id.to_string(),
        delivery_binding,
    };
    let stored = StoredParentResolution {
        preview_id: preview_id.as_str().to_string(),
        parent_actor_id: actor_id.to_string(),
        issued_at: Instant::now(),
        in_flight: true,
    };
    let mut resolution_store = store()
        .lock()
        .map_err(|_| "parent resolution store is unavailable".to_string())?;
    let now = Instant::now();
    resolution_store
        .entries
        .retain(|_, entry| now.duration_since(entry.issued_at) <= STAGED_RESOLUTION_TTL);
    if resolution_store.entries.values().any(|entry| {
        entry.preview_id == stored.preview_id && entry.parent_actor_id == stored.parent_actor_id
    }) {
        return Err(
            "parent resolution already has a relay attempt; refresh for manual review".to_string(),
        );
    }
    while resolution_store.entries.len() >= MAX_STAGED_RESOLUTIONS {
        let Some(oldest_handle) = resolution_store
            .entries
            .iter()
            .min_by_key(|(_, entry)| entry.issued_at)
            .map(|(handle, _)| handle.clone())
        else {
            break;
        };
        resolution_store.entries.remove(&oldest_handle);
    }
    resolution_store.entries.insert(handle.clone(), stored);

    Ok(StagedParentResolution {
        handle,
        preview_id: preview_id.as_str().to_string(),
        parent_actor_id: actor_id.to_string(),
        request,
    })
}

pub(super) fn request_payload(staged: &StagedParentResolution) -> Result<Value, String> {
    let request = serde_json::to_string(&staged.request)
        .map_err(|_| "parent resolution request could not be serialized".to_string())?;
    Ok(serde_json::json!({ RESOLUTION_PAYLOAD_FIELD: request }))
}

pub(super) fn commit(staged: &StagedParentResolution) -> Result<(), String> {
    let mut resolution_store = store()
        .lock()
        .map_err(|_| "parent resolution store is unavailable".to_string())?;
    let Some(entry) = resolution_store.entries.get(&staged.handle) else {
        return Err("parent resolution handle is unknown or already consumed".to_string());
    };
    if entry.preview_id != staged.preview_id || entry.parent_actor_id != staged.parent_actor_id {
        return Err("parent resolution handle context does not match".to_string());
    }
    if !entry.in_flight {
        return Err("parent resolution is not pending".to_string());
    }
    resolution_store.entries.remove(&staged.handle);
    Ok(())
}

pub(super) fn restore(staged: &StagedParentResolution) -> Result<(), String> {
    let mut resolution_store = store()
        .lock()
        .map_err(|_| "parent resolution store is unavailable".to_string())?;
    let Some(entry) = resolution_store.entries.get_mut(&staged.handle) else {
        return Err("parent resolution handle is unknown or already consumed".to_string());
    };
    if entry.preview_id != staged.preview_id || entry.parent_actor_id != staged.parent_actor_id {
        return Err("parent resolution handle context does not match".to_string());
    }
    entry.in_flight = false;
    Ok(())
}

fn decision_from_payload(payload: &Value) -> Result<PolicyRequestParentResolutionDecision, String> {
    let object = payload
        .as_object()
        .ok_or_else(|| "parent resolution payload must be an object".to_string())?;
    if object.len() != 1 || !object.contains_key(RESOLUTION_PAYLOAD_FIELD) {
        return Err("parent resolution payload must contain only the decision field".to_string());
    }
    let value = object
        .get(RESOLUTION_PAYLOAD_FIELD)
        .ok_or_else(|| "parent resolution decision is missing".to_string())?;
    let value = match value {
        Value::String(text) => serde_json::from_str(text)
            .map_err(|_| "parent resolution decision payload is invalid".to_string())?,
        value => value.clone(),
    };
    let input: ParentResolutionDecisionInput = serde_json::from_value(value)
        .map_err(|_| "parent resolution payload must contain only a valid decision".to_string())?;
    Ok(input.decision)
}

fn approved_action(
    read_model: &ParentPolicyPreviewReadModelSnapshot,
) -> Result<PolicyRequestAssistantPreviewConfirmAction, String> {
    let value = read_model
        .network_mapped_policy_action
        .as_deref()
        .or(read_model.network_requested_policy_action.as_deref())
        .ok_or_else(|| {
            "parent resolution approved action is unavailable; manual review required".to_string()
        })?;
    serde_json::from_value(Value::String(value.to_string())).map_err(|_| {
        "parent resolution approved action is unsupported; manual review required".to_string()
    })
}

fn actor_role(
    value: &Option<String>,
) -> Result<PolicyRequestAssistantPreviewConfirmActorRole, String> {
    let value = required_context(value, "actor role")?;
    serde_json::from_value(Value::String(value.to_string())).map_err(|_| {
        "parent resolution actor role is unavailable; manual review required".to_string()
    })
}

fn actor_state(
    value: &Option<String>,
) -> Result<PolicyRequestAssistantPreviewConfirmActorState, String> {
    let value = required_context(value, "actor state")?;
    serde_json::from_value(Value::String(value.to_string())).map_err(|_| {
        "parent resolution actor state is unavailable; manual review required".to_string()
    })
}

fn local_controller_actor_id(
    read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> Result<&str, String> {
    let read_model = read_model.ok_or_else(|| {
        "local controller authority is unavailable; manual review required".to_string()
    })?;
    let mut actor_ids = read_model
        .household_device_decisions
        .iter()
        .filter(|decision| decision.revoked_at.is_none())
        .map(|decision| decision.parent_actor_id.as_str())
        .collect::<Vec<_>>();
    actor_ids.sort_unstable();
    actor_ids.dedup();
    match actor_ids.as_slice() {
        [actor_id] => Ok(*actor_id),
        _ => Err(
            "local controller actor is unavailable or ambiguous; manual review required"
                .to_string(),
        ),
    }
}

fn single_audit_reference(value: Option<&str>) -> Result<&str, String> {
    let value = value
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            "parent resolution approval audit reference is unavailable; manual review required"
                .to_string()
        })?;
    let references = value
        .split(constants::delimiter::LIST)
        .filter(|value| !value.trim().is_empty())
        .collect::<Vec<_>>();
    match references.as_slice() {
        [reference] => Ok(*reference),
        _ => Err(
            "parent resolution requires one exact approval audit reference; manual review required"
                .to_string(),
        ),
    }
}

fn required_context<'a>(value: &'a Option<String>, label: &str) -> Result<&'a str, String> {
    value
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| format!("parent resolution {label} is unavailable; manual review required"))
}

fn opaque_handle() -> Result<String, String> {
    for _ in 0..4 {
        let mut random = [0_u8; 24];
        getrandom::fill(&mut random)
            .map_err(|_| "parent resolution handle entropy is unavailable".to_string())?;
        let handle = format!(
            "pprh-{}",
            random
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>()
        );
        if store()
            .lock()
            .map_err(|_| "parent resolution store is unavailable".to_string())?
            .entries
            .get(&handle)
            .is_none()
        {
            return Ok(handle);
        }
    }
    Err("parent resolution handle generation collided".to_string())
}
