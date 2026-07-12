use ocentra_parent_agent_protocol::app_game::{AppGameServiceReadModel, APP_GAME_SCHEMA_VERSION};
use ocentra_parent_agent_protocol::app_game_authority_classifier::{
    AppGameControlActionResult, APP_GAME_CONTROL_ACTION_STATUS_ENFORCED,
    APP_GAME_CONTROL_POLICY_KIND_GAME, APP_GAME_ENFORCEMENT_RESULT_ACTUALLY_ENFORCED,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::AppGameTimerParentSurfaceChildUxLocalArtifactRecord;
use ocentra_parent_agent_protocol::AppGameTimerParentSurfaceChildUxParentPreferenceSetupRecord;
use ocentra_parent_agent_protocol::AppGameTimerParentSurfaceChildUxParentSurfaceIntentRecord;
use ocentra_parent_agent_protocol::APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP;
use ocentra_parent_agent_protocol::APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME;

struct ChildUxPrefix(&'static str);

struct ChildUxReferenceId(String);

struct ChildUxReferenceIds(Vec<String>);

struct ChildUxTargetDomain(String);

struct ActionResultValueInput(Vec<String>);

pub(crate) struct TimerParentSurfaceControlActionResults {
    pub(crate) reference_ids: Vec<String>,
    pub(crate) statuses: Vec<String>,
    pub(crate) capability_states: Vec<String>,
    pub(crate) enforcement_statuses: Vec<String>,
    pub(crate) child_reason_reference_ids: Vec<String>,
    pub(crate) child_status_reference_ids: Vec<String>,
    pub(crate) child_ux_handoff_ready_count: u64,
    pub(crate) child_ux_handoff_blocked_count: u64,
    pub(crate) child_ux_handoff_reference_ids: Vec<String>,
    pub(crate) child_ux_local_handoff_artifact_record_count: u64,
    pub(crate) child_ux_local_handoff_artifact_skipped_count: u64,
    pub(crate) child_ux_local_handoff_artifact_reference_ids: Vec<String>,
    pub(crate) child_ux_local_handoff_artifact_records:
        Vec<AppGameTimerParentSurfaceChildUxLocalArtifactRecord>,
    pub(crate) child_ux_parent_surface_intent_manual_action_required_count: u64,
    pub(crate) child_ux_parent_surface_intent_unavailable_visible_count: u64,
    pub(crate) child_ux_parent_surface_intent_history_visible_count: u64,
    pub(crate) child_ux_parent_surface_intent_preference_setup_required_count: u64,
    pub(crate) child_ux_parent_surface_intent_reference_ids: Vec<String>,
    pub(crate) child_ux_parent_surface_intent_records:
        Vec<AppGameTimerParentSurfaceChildUxParentSurfaceIntentRecord>,
    pub(crate) child_ux_parent_preference_setup_draft_ready_count: u64,
    pub(crate) child_ux_parent_preference_setup_unavailable_visible_count: u64,
    pub(crate) child_ux_parent_preference_setup_reference_ids: Vec<String>,
    pub(crate) child_ux_parent_preference_setup_request_ready_count: u64,
    pub(crate) child_ux_parent_preference_setup_request_unavailable_visible_count: u64,
    pub(crate) child_ux_parent_preference_setup_request_reference_ids: Vec<String>,
    pub(crate) child_ux_parent_preference_setup_records:
        Vec<AppGameTimerParentSurfaceChildUxParentPreferenceSetupRecord>,
    pub(crate) adapter_dispatch_claimed: bool,
    pub(crate) platform_enforcement_claimed: bool,
}

struct ActionResultValues {
    reference_ids: Vec<String>,
    statuses: Vec<String>,
    capability_states: Vec<String>,
    enforcement_statuses: Vec<String>,
    child_reason_reference_ids: Vec<String>,
    child_status_reference_ids: Vec<String>,
    adapter_dispatch_claimed: bool,
    platform_enforcement_claimed: bool,
}

pub(crate) fn timer_parent_surface_control_action_results(
    model: &AppGameServiceReadModel,
) -> TimerParentSurfaceControlActionResults {
    let action_result_values = action_result_values(model);
    let child_ux_handoff_reference_ids = child_ux_handoff_reference_ids(model);
    let child_ux_handoff_ready_count = child_ux_handoff_reference_ids.0.len() as u64;
    let child_ux_handoff_blocked_count =
        model.approval_action_result_rows.len() as u64 - child_ux_handoff_ready_count;
    let child_ux_local_handoff_artifact_reference_ids = child_ux_reference_ids(
        ChildUxPrefix(constants::value::APP_GAME_CHILD_UX_LOCAL_HANDOFF_ARTIFACT_PREFIX),
        &child_ux_handoff_reference_ids,
    );
    let child_ux_local_handoff_artifact_records = child_ux_local_handoff_artifact_records(model);
    let child_ux_parent_surface_intent_reference_ids = child_ux_reference_ids(
        ChildUxPrefix(constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX),
        &child_ux_handoff_reference_ids,
    );
    let child_ux_parent_surface_intent_records =
        child_ux_parent_surface_intent_records(&child_ux_local_handoff_artifact_records);
    let child_ux_parent_surface_intent_ready_count =
        child_ux_parent_surface_intent_reference_ids.0.len() as u64;
    let child_ux_parent_preference_setup_reference_ids = child_ux_reference_ids(
        ChildUxPrefix(constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX),
        &child_ux_handoff_reference_ids,
    );
    let child_ux_parent_preference_setup_records =
        child_ux_parent_preference_setup_records(&child_ux_parent_surface_intent_records);

    TimerParentSurfaceControlActionResults {
        reference_ids: action_result_values.reference_ids,
        statuses: action_result_values.statuses,
        capability_states: action_result_values.capability_states,
        enforcement_statuses: action_result_values.enforcement_statuses,
        child_reason_reference_ids: action_result_values.child_reason_reference_ids,
        child_status_reference_ids: action_result_values.child_status_reference_ids,
        child_ux_handoff_ready_count,
        child_ux_handoff_blocked_count,
        child_ux_handoff_reference_ids: child_ux_handoff_reference_ids.0,
        child_ux_local_handoff_artifact_record_count: child_ux_local_handoff_artifact_reference_ids
            .0
            .len() as u64,
        child_ux_local_handoff_artifact_skipped_count: child_ux_handoff_blocked_count,
        child_ux_local_handoff_artifact_reference_ids:
            child_ux_local_handoff_artifact_reference_ids.0,
        child_ux_local_handoff_artifact_records,
        child_ux_parent_surface_intent_manual_action_required_count:
            child_ux_parent_surface_intent_ready_count,
        child_ux_parent_surface_intent_unavailable_visible_count: 0,
        child_ux_parent_surface_intent_history_visible_count:
            child_ux_parent_surface_intent_ready_count,
        child_ux_parent_surface_intent_preference_setup_required_count:
            child_ux_parent_surface_intent_ready_count,
        child_ux_parent_surface_intent_reference_ids: child_ux_parent_surface_intent_reference_ids
            .0,
        child_ux_parent_surface_intent_records,
        child_ux_parent_preference_setup_draft_ready_count:
            child_ux_parent_preference_setup_reference_ids.0.len() as u64,
        child_ux_parent_preference_setup_unavailable_visible_count: 0,
        child_ux_parent_preference_setup_request_ready_count:
            child_ux_parent_preference_setup_reference_ids.0.len() as u64,
        child_ux_parent_preference_setup_request_unavailable_visible_count: 0,
        child_ux_parent_preference_setup_request_reference_ids:
            child_ux_parent_preference_setup_reference_ids.0.clone(),
        child_ux_parent_preference_setup_reference_ids:
            child_ux_parent_preference_setup_reference_ids.0,
        child_ux_parent_preference_setup_records,
        adapter_dispatch_claimed: action_result_values.adapter_dispatch_claimed,
        platform_enforcement_claimed: action_result_values.platform_enforcement_claimed,
    }
}

fn action_result_values(model: &AppGameServiceReadModel) -> ActionResultValues {
    ActionResultValues {
        reference_ids: model
            .approval_action_result_rows
            .iter()
            .map(|row| row.result_id.clone())
            .collect(),
        statuses: unique_action_result_values(ActionResultValueInput(
            model
                .approval_action_result_rows
                .iter()
                .map(|row| row.result_status.clone())
                .collect(),
        ))
        .0,
        capability_states: unique_action_result_values(ActionResultValueInput(
            model
                .approval_action_result_rows
                .iter()
                .map(|row| row.capability_state.clone())
                .collect(),
        ))
        .0,
        enforcement_statuses: unique_action_result_values(ActionResultValueInput(
            model
                .approval_action_result_rows
                .iter()
                .filter_map(|row| row.enforcement_result.as_ref())
                .map(|result| result.status.clone())
                .collect(),
        ))
        .0,
        child_reason_reference_ids: unique_action_result_values(ActionResultValueInput(
            model
                .approval_action_result_rows
                .iter()
                .flat_map(|row| row.request.child_reason_references.iter().cloned())
                .collect(),
        ))
        .0,
        child_status_reference_ids: unique_action_result_values(ActionResultValueInput(
            model
                .approval_action_result_rows
                .iter()
                .flat_map(|row| row.request.child_status_references.iter().cloned())
                .collect(),
        ))
        .0,
        adapter_dispatch_claimed: model
            .approval_action_result_rows
            .iter()
            .any(|row| row.result_status == APP_GAME_CONTROL_ACTION_STATUS_ENFORCED),
        platform_enforcement_claimed: model.approval_action_result_rows.iter().any(|row| {
            row.enforcement_result.as_ref().is_some_and(|result| {
                result.status == APP_GAME_ENFORCEMENT_RESULT_ACTUALLY_ENFORCED
            })
        }),
    }
}

fn child_ux_handoff_reference_ids(model: &AppGameServiceReadModel) -> ChildUxReferenceIds {
    ChildUxReferenceIds(
        model
            .approval_action_result_rows
            .iter()
            .filter(|row| child_ux_local_artifact_row_is_ready(row))
            .map(|row| row.result_id.clone())
            .collect(),
    )
}

fn child_ux_reference_ids(
    prefix: ChildUxPrefix,
    reference_ids: &ChildUxReferenceIds,
) -> ChildUxReferenceIds {
    ChildUxReferenceIds(
        reference_ids
            .0
            .iter()
            .map(|reference_id| {
                child_ux_reference_id(
                    ChildUxPrefix(prefix.0),
                    &ChildUxReferenceId(reference_id.clone()),
                )
                .0
            })
            .collect(),
    )
}

fn child_ux_reference_id(
    prefix: ChildUxPrefix,
    reference_id: &ChildUxReferenceId,
) -> ChildUxReferenceId {
    let mut child_ux_reference_id = String::from(prefix.0);
    child_ux_reference_id.push_str(&reference_id.0);
    ChildUxReferenceId(child_ux_reference_id)
}

fn child_ux_local_artifact_reference_id(reference_id: &ChildUxReferenceId) -> ChildUxReferenceId {
    let mut artifact_reference_id =
        String::from(constants::value::APP_GAME_CHILD_UX_LOCAL_HANDOFF_ARTIFACT_PREFIX);
    artifact_reference_id.push_str(&reference_id.0);
    ChildUxReferenceId(artifact_reference_id)
}

fn child_ux_local_artifact_target_domain(row: &AppGameControlActionResult) -> ChildUxTargetDomain {
    if row.request.policy_kind == APP_GAME_CONTROL_POLICY_KIND_GAME {
        ChildUxTargetDomain(APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME.to_string())
    } else {
        ChildUxTargetDomain(APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP.to_string())
    }
}

fn unique_action_result_values(values: ActionResultValueInput) -> ChildUxReferenceIds {
    let mut unique = Vec::new();
    for value in values.0 {
        if !value.is_empty() && !unique.iter().any(|existing| existing == &value) {
            unique.push(value);
        }
    }
    ChildUxReferenceIds(unique)
}

fn child_ux_parent_preference_setup_request_refs(
    record: &AppGameTimerParentSurfaceChildUxParentSurfaceIntentRecord,
) -> ChildUxReferenceIds {
    let mut refs = vec![record.parent_surface_intent_reference_id.clone()];
    refs.extend(record.drill_in_reference_ids.clone());
    refs.extend(record.manual_proof_reference_ids.clone());
    unique_action_result_values(ActionResultValueInput(refs))
}

fn child_ux_parent_surface_drill_in_refs(
    record: &AppGameTimerParentSurfaceChildUxLocalArtifactRecord,
) -> ChildUxReferenceIds {
    let mut refs = vec![record.artifact_reference_id.clone()];
    refs.extend(record.child_reason_reference_ids.clone());
    refs.extend(record.child_status_reference_ids.clone());
    ChildUxReferenceIds(refs)
}

fn child_ux_parent_surface_manual_proof_refs(
    record: &AppGameTimerParentSurfaceChildUxLocalArtifactRecord,
) -> ChildUxReferenceIds {
    let mut refs = record.child_reason_reference_ids.clone();
    refs.extend(record.child_status_reference_ids.clone());
    ChildUxReferenceIds(refs)
}

fn child_ux_local_handoff_artifact_records(
    model: &AppGameServiceReadModel,
) -> Vec<AppGameTimerParentSurfaceChildUxLocalArtifactRecord> {
    model
        .approval_action_result_rows
        .iter()
        .filter(|row| child_ux_local_artifact_row_is_ready(row))
        .map(child_ux_local_artifact_record)
        .collect()
}

fn child_ux_parent_surface_intent_records(
    records: &[AppGameTimerParentSurfaceChildUxLocalArtifactRecord],
) -> Vec<AppGameTimerParentSurfaceChildUxParentSurfaceIntentRecord> {
    records
        .iter()
        .map(child_ux_parent_surface_intent_record)
        .collect()
}

fn child_ux_parent_surface_intent_record(
    record: &AppGameTimerParentSurfaceChildUxLocalArtifactRecord,
) -> AppGameTimerParentSurfaceChildUxParentSurfaceIntentRecord {
    AppGameTimerParentSurfaceChildUxParentSurfaceIntentRecord {
        schema_version: APP_GAME_SCHEMA_VERSION,
        parent_surface_intent_reference_id: child_ux_reference_id(
            ChildUxPrefix(constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX),
            &ChildUxReferenceId(record.source_result_id.clone()),
        )
        .0,
        source_result_id: record.source_result_id.clone(),
        source_artifact_reference_id: record.artifact_reference_id.clone(),
        target_domain: record.target_domain.clone(),
        history_visibility: String::from(
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_HISTORY_ROW_VISIBLE,
        ),
        parent_surface_status: String::from(
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_MANUAL_ACTION_REQUIRED,
        ),
        preference_visibility: String::from(
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_PREFERENCE_SETUP_REQUIRED,
        ),
        drill_in_reference_ids: child_ux_parent_surface_drill_in_refs(record).0,
        manual_proof_reference_ids: child_ux_parent_surface_manual_proof_refs(record).0,
        sensitive_detail_included: false,
        parent_notification_ui_rendered: false,
        parent_preference_mutation_claimed: false,
        provider_delivery_claimed: false,
        child_delivery_claimed: false,
        adapter_dispatch_claimed: record.adapter_dispatch_claimed,
        platform_enforcement_claimed: record.platform_enforcement_claimed,
        raw_private_source_rows_included: false,
    }
}

fn child_ux_parent_preference_setup_records(
    records: &[AppGameTimerParentSurfaceChildUxParentSurfaceIntentRecord],
) -> Vec<AppGameTimerParentSurfaceChildUxParentPreferenceSetupRecord> {
    records
        .iter()
        .map(child_ux_parent_preference_setup_record)
        .collect()
}

fn child_ux_parent_preference_setup_record(
    record: &AppGameTimerParentSurfaceChildUxParentSurfaceIntentRecord,
) -> AppGameTimerParentSurfaceChildUxParentPreferenceSetupRecord {
    AppGameTimerParentSurfaceChildUxParentPreferenceSetupRecord {
        schema_version: APP_GAME_SCHEMA_VERSION,
        parent_preference_setup_reference_id: child_ux_reference_id(
            ChildUxPrefix(constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX),
            &ChildUxReferenceId(record.source_result_id.clone()),
        )
        .0,
        source_parent_surface_intent_reference_id: record
            .parent_surface_intent_reference_id
            .clone(),
        source_result_id: record.source_result_id.clone(),
        source_artifact_reference_id: record.source_artifact_reference_id.clone(),
        target_domain: record.target_domain.clone(),
        draft_status: String::from(
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_DRAFT_READY,
        ),
        parent_preference_setup_request_status: String::from(
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_REQUEST_READY,
        ),
        parent_preference_setup_request_reference_ids:
            child_ux_parent_preference_setup_request_refs(record).0,
        drill_in_reference_ids: record.drill_in_reference_ids.clone(),
        manual_proof_reference_ids: record.manual_proof_reference_ids.clone(),
        parent_preference_ui_rendered: false,
        parent_frequency_control_ui_rendered: false,
        parent_preference_mutation_claimed: false,
        notification_rule_mutation_claimed: false,
        provider_delivery_claimed: false,
        child_delivery_claimed: false,
        adapter_dispatch_claimed: record.adapter_dispatch_claimed,
        platform_enforcement_claimed: record.platform_enforcement_claimed,
        raw_private_source_rows_included: false,
    }
}

fn child_ux_local_artifact_row_is_ready(row: &AppGameControlActionResult) -> bool {
    !row.request.child_reason_references.is_empty()
        && !row.request.child_status_references.is_empty()
}

fn child_ux_local_artifact_record(
    row: &AppGameControlActionResult,
) -> AppGameTimerParentSurfaceChildUxLocalArtifactRecord {
    let adapter_dispatch_claimed = row.result_status == APP_GAME_CONTROL_ACTION_STATUS_ENFORCED;
    let platform_enforcement_claimed = row
        .enforcement_result
        .as_ref()
        .is_some_and(|result| result.status == APP_GAME_ENFORCEMENT_RESULT_ACTUALLY_ENFORCED);

    AppGameTimerParentSurfaceChildUxLocalArtifactRecord {
        schema_version: APP_GAME_SCHEMA_VERSION,
        artifact_reference_id: child_ux_local_artifact_reference_id(&ChildUxReferenceId(
            row.result_id.clone(),
        ))
        .0,
        source_result_id: row.result_id.clone(),
        target_domain: child_ux_local_artifact_target_domain(row).0,
        child_reason_reference_ids: row.request.child_reason_references.clone(),
        child_status_reference_ids: row.request.child_status_references.clone(),
        child_delivery_claimed: false,
        notification_delivery_claimed: false,
        adapter_dispatch_claimed,
        platform_enforcement_claimed,
        raw_private_source_rows_included: false,
    }
}
