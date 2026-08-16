use ocentra_parent_agent_protocol::app_game_timer_parent_preference_setup_request::AppGameTimerParentPreferenceSetupRequestResult;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::{AgentEventEnvelope, AgentEventName};
use ocentra_schema::parent_ui_bridge::{
    ParentCommandResultDetailSnapshot, ParentCommandResultProjectionSnapshot,
};

const APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_PROJECTION: &str =
    "app-game-timer-parent-preference-setup";
const DETAIL_SEPARATOR: &str = " | ";
const PARENT_PREFERENCE_SETUP_STATUS_LABELS: &[(&str, &str)] = &[
    ("handoff-ready", "Handoff ready"),
    ("persisted", "Persisted"),
    ("accepted", "Ready"),
    ("queued", "Ready"),
    ("dispatch-ready", "Ready"),
    ("receipt-required", "Ready"),
    ("receipt-pending", "Ready"),
    ("receipt-ingested", "Ready"),
    ("outbox-recorded", "Ready"),
    ("provider-delivery-queued", "Ready"),
    ("provider-manual-required", "Manual required"),
    ("provider-delivery-manual-required", "Manual required"),
    ("provider-adapter-required", "Manual required"),
    ("provider-credential-proof-required", "Manual required"),
    ("provider-delivery-receipt-required", "Required"),
    ("provider-delivery-receipt-pending", "Pending"),
    ("provider-delivery-receipt-ingested", "Ready"),
    ("unavailable", "Unavailable"),
];

pub(super) fn command_result_projection(
    event: &AgentEventEnvelope,
) -> Option<ParentCommandResultProjectionSnapshot> {
    if event.event != AgentEventName::AgentActivityAppGameTimerParentPreferenceSetupRequested {
        return None;
    }

    Some(ParentCommandResultProjectionSnapshot {
        projection_kind: APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_PROJECTION.to_string(),
        details: timer_parent_preference_setup_details(event),
    })
}

fn timer_parent_preference_setup_details(
    event: &AgentEventEnvelope,
) -> Vec<ParentCommandResultDetailSnapshot> {
    match timer_parent_preference_setup_result(event) {
        Ok(result) => timer_parent_preference_setup_result_details(&result),
        Err(reason) => vec![detail("Status", "Review"), detail("Reason", reason)],
    }
}

fn timer_parent_preference_setup_result(
    event: &AgentEventEnvelope,
) -> Result<AppGameTimerParentPreferenceSetupRequestResult, &'static str> {
    let raw = match event
        .payload
        .get(constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUEST)
    {
        Some(LogFieldValue::String(value)) => value.as_str(),
        _ => return Err("missing-json-field"),
    };

    serde_json::from_str(raw).map_err(|error| {
        if error.is_syntax() || error.is_eof() {
            "invalid-json"
        } else {
            "invalid-payload"
        }
    })
}

fn timer_parent_preference_setup_result_details(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> Vec<ParentCommandResultDetailSnapshot> {
    let mut details = timer_parent_preference_setup_primary_details(result);
    details.extend(timer_parent_preference_setup_child_runtime_details(result));
    details.extend(timer_parent_preference_setup_no_claim_details(result));
    details
}

fn timer_parent_preference_setup_primary_details(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> Vec<ParentCommandResultDetailSnapshot> {
    vec![
        detail("Status", "Ready"),
        detail("Event ID", &result.request_id),
        detail("Parent preference setup accepted at", &result.accepted_at),
        detail(
            "Parent preference setup request refs",
            &joined_or_not_reported(&result.request_reference_ids),
        ),
        detail(
            "Parent preference setup action-result refs",
            &joined_or_not_reported(&result.action_result_reference_ids),
        ),
        detail(
            "Parent preference setup action-result status",
            &parent_preference_setup_result_status(&result.action_result_persistence_status),
        ),
        detail(
            "Parent preference setup mutation receipt refs",
            &joined_or_not_reported(&result.parent_preference_mutation_receipt_ids),
        ),
        detail(
            "Parent preference setup mutation receipt status",
            &parent_preference_setup_result_status(
                &result.parent_preference_mutation_receipt_status,
            ),
        ),
    ]
}

fn timer_parent_preference_setup_child_runtime_details(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> Vec<ParentCommandResultDetailSnapshot> {
    let mut details = vec![
        refs_detail(
            "Child runtime handoff refs",
            &result.child_runtime_delivery_handoff_ids,
        ),
        status_detail(
            "Child runtime handoff status",
            &result.child_runtime_delivery_handoff_status,
        ),
        refs_detail(
            "Child runtime queue refs",
            &result.child_runtime_delivery_queue_ids,
        ),
        status_detail(
            "Child runtime queue status",
            &result.child_runtime_delivery_queue_status,
        ),
        refs_detail(
            "Child runtime dispatch refs",
            &result.child_runtime_delivery_dispatch_ids,
        ),
        status_detail(
            "Child runtime dispatch status",
            &result.child_runtime_delivery_dispatch_status,
        ),
        refs_detail(
            "Child runtime receipt-required refs",
            &result.child_runtime_delivery_receipt_requirement_ids,
        ),
        status_detail(
            "Child runtime receipt-required status",
            &result.child_runtime_delivery_receipt_requirement_status,
        ),
        refs_detail(
            "Child runtime receipt-pending refs",
            &result.child_runtime_delivery_receipt_pending_ids,
        ),
        status_detail(
            "Child runtime receipt-pending status",
            &result.child_runtime_delivery_receipt_pending_status,
        ),
    ];
    details.extend(timer_parent_preference_setup_child_runtime_tail_details(
        result,
    ));
    details
}

fn timer_parent_preference_setup_child_runtime_tail_details(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> Vec<ParentCommandResultDetailSnapshot> {
    let mut details = vec![
        refs_detail(
            "Child runtime receipt-ingested refs",
            &result.child_runtime_delivery_receipt_ingested_ids,
        ),
        status_detail(
            "Child runtime receipt-ingested status",
            &result.child_runtime_delivery_receipt_ingested_status,
        ),
        refs_detail(
            "Durable local outbox refs",
            &result.durable_outbox_record_ids,
        ),
        status_detail("Durable local outbox status", &result.durable_outbox_status),
    ];
    details.extend(timer_parent_preference_setup_provider_details(result));
    details
}

fn timer_parent_preference_setup_provider_details(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> Vec<ParentCommandResultDetailSnapshot> {
    let mut details = timer_parent_preference_setup_provider_aggregate_details(result);
    details.extend(timer_parent_preference_setup_provider_queue_details(result));
    details.extend(timer_parent_preference_setup_provider_receipt_details(
        result,
    ));
    details
}

fn timer_parent_preference_setup_provider_aggregate_details(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> Vec<ParentCommandResultDetailSnapshot> {
    let aggregate_status = if result.provider_delivery_claimed
        || result.provider_receipt_ingestion_claimed
    {
        "Review"
    } else {
        "Manual provider setup required; local outbox, queue, and receipt tracking are recorded."
    };

    vec![
        detail("Provider delivery aggregate status", aggregate_status),
        detail(
            "Provider delivery next action",
            "Configure provider adapter and credential proof before external delivery.",
        ),
        detail(
            "Provider delivery proof state",
            "Local durable outbox, provider queue, receipt-required, pending, and ingested refs are visible.",
        ),
        detail(
            "Provider delivery no-claim boundary",
            "Provider delivery execution and external provider receipt ingestion are not claimed.",
        ),
    ]
}

fn timer_parent_preference_setup_provider_queue_details(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> Vec<ParentCommandResultDetailSnapshot> {
    vec![
        refs_detail(
            "Provider delivery readiness refs",
            &result.provider_delivery_readiness_ids,
        ),
        status_detail(
            "Provider delivery readiness status",
            &result.provider_delivery_readiness_status,
        ),
        refs_detail(
            "Provider delivery attempt refs",
            &result.provider_delivery_attempt_ids,
        ),
        status_detail(
            "Provider delivery attempt status",
            &result.provider_delivery_attempt_status,
        ),
        refs_detail(
            "Provider delivery adapter requirement refs",
            &result.provider_delivery_adapter_requirement_ids,
        ),
        status_detail(
            "Provider delivery adapter requirement status",
            &result.provider_delivery_adapter_requirement_status,
        ),
        refs_detail(
            "Provider delivery credential requirement refs",
            &result.provider_delivery_credential_requirement_ids,
        ),
        status_detail(
            "Provider delivery credential requirement status",
            &result.provider_delivery_credential_requirement_status,
        ),
        refs_detail(
            "Provider delivery queue refs",
            &result.provider_delivery_queue_ids,
        ),
        status_detail(
            "Provider delivery queue status",
            &result.provider_delivery_queue_status,
        ),
    ]
}

fn timer_parent_preference_setup_provider_receipt_details(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> Vec<ParentCommandResultDetailSnapshot> {
    vec![
        refs_detail(
            "Provider delivery receipt-required refs",
            &result.provider_delivery_receipt_requirement_ids,
        ),
        status_detail(
            "Provider delivery receipt-required status",
            &result.provider_delivery_receipt_requirement_status,
        ),
        refs_detail(
            "Provider delivery receipt-pending refs",
            &result.provider_delivery_receipt_pending_ids,
        ),
        status_detail(
            "Provider delivery receipt-pending status",
            &result.provider_delivery_receipt_pending_status,
        ),
        refs_detail(
            "Provider delivery receipt-ingested refs",
            &result.provider_delivery_receipt_ingested_ids,
        ),
        status_detail(
            "Provider delivery receipt-ingested status",
            &result.provider_delivery_receipt_ingested_status,
        ),
    ]
}

fn timer_parent_preference_setup_no_claim_details(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> Vec<ParentCommandResultDetailSnapshot> {
    vec![
        detail("Parent preference setup mutation", "Not claimed"),
        detail("Notification rule mutation", "Not claimed"),
        detail(
            "Child delivery",
            &claimed_value(result.child_runtime_delivery_claimed),
        ),
        detail(
            "Adapter dispatch",
            &claimed_value(result.adapter_dispatch_claimed),
        ),
        detail(
            "Platform state",
            &claimed_value(result.platform_enforcement_claimed),
        ),
    ]
}

fn refs_detail(label: &str, values: &[String]) -> ParentCommandResultDetailSnapshot {
    detail(label, &joined_or_not_reported(values))
}

fn status_detail(label: &str, status: &str) -> ParentCommandResultDetailSnapshot {
    detail(label, &parent_preference_setup_result_status(status))
}

fn parent_preference_setup_result_status(status: &str) -> String {
    PARENT_PREFERENCE_SETUP_STATUS_LABELS
        .iter()
        .find(|(raw, _)| *raw == status)
        .map(|(_, label)| (*label).to_string())
        .unwrap_or_else(|| status.to_string())
}

fn claimed_value(value: bool) -> String {
    if value {
        "Ready".to_string()
    } else {
        "Not claimed".to_string()
    }
}

fn joined_or_not_reported(values: &[String]) -> String {
    if values.is_empty() {
        return "Not reported".to_string();
    }
    values.join(DETAIL_SEPARATOR)
}

fn detail(label: &str, value: &str) -> ParentCommandResultDetailSnapshot {
    ParentCommandResultDetailSnapshot {
        label: label.to_string(),
        value: value.to_string(),
    }
}
