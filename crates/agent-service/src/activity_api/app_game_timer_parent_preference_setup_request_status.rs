use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::AppGameTimerParentPreferenceSetupRequestResult;

pub(crate) fn apply_persisted_setup_statuses(
    result: &mut AppGameTimerParentPreferenceSetupRequestResult,
) {
    apply_persisted_action_statuses(result);
    apply_persisted_child_runtime_statuses(result);
    apply_persisted_provider_statuses(result);
}

fn apply_persisted_action_statuses(result: &mut AppGameTimerParentPreferenceSetupRequestResult) {
    result.action_result_persistence_status =
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_ACTION_RESULT_PERSISTED.to_string();
    result.action_result_persistence_claimed = true;
    result.parent_preference_mutation_receipt_status =
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT_PERSISTED.to_string();
    result.parent_preference_mutation_receipt_claimed = true;
    result.durable_outbox_status =
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_DURABLE_OUTBOX_RECORDED.to_string();
    result.durable_outbox_claimed = true;
}

fn apply_persisted_child_runtime_statuses(
    result: &mut AppGameTimerParentPreferenceSetupRequestResult,
) {
    result.child_runtime_delivery_handoff_status =
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF_READY
            .to_string();
    result.child_runtime_delivery_handoff_claimed = true;
    result.child_runtime_delivery_queue_status =
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_QUEUE_QUEUED
            .to_string();
    result.child_runtime_delivery_queue_claimed = true;
    result.child_runtime_delivery_dispatch_status =
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_DISPATCH_READY
            .to_string();
    result.child_runtime_delivery_dispatch_claimed = true;
    apply_persisted_child_runtime_receipt_statuses(result);
}

fn apply_persisted_child_runtime_receipt_statuses(
    result: &mut AppGameTimerParentPreferenceSetupRequestResult,
) {
    result.child_runtime_delivery_receipt_requirement_status =
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_REQUIRED
            .to_string();
    result.child_runtime_delivery_receipt_requirement_claimed = true;
    result.child_runtime_delivery_receipt_pending_status =
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_PENDING
            .to_string();
    result.child_runtime_delivery_receipt_pending_claimed = true;
    result.child_runtime_delivery_receipt_ingested_status =
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_PENDING
            .to_string();
    result.child_runtime_delivery_receipt_ingested_claimed = false;
}

fn apply_persisted_provider_statuses(result: &mut AppGameTimerParentPreferenceSetupRequestResult) {
    result.provider_delivery_readiness_status =
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_MANUAL_REQUIRED
            .to_string();
    result.provider_delivery_readiness_claimed = true;
    result.provider_delivery_attempt_status =
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_ATTEMPT_MANUAL_REQUIRED
            .to_string();
    result.provider_delivery_attempt_claimed = true;
    result.provider_delivery_adapter_requirement_status =
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_ADAPTER_REQUIRED
            .to_string();
    result.provider_delivery_adapter_requirement_claimed = true;
    apply_persisted_provider_delivery_statuses(result);
}

fn apply_persisted_provider_delivery_statuses(
    result: &mut AppGameTimerParentPreferenceSetupRequestResult,
) {
    result.provider_delivery_credential_requirement_status =
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_CREDENTIAL_PROOF_REQUIRED
            .to_string();
    result.provider_delivery_credential_requirement_claimed = true;
    result.provider_delivery_queue_status =
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_QUEUE_QUEUED
            .to_string();
    result.provider_delivery_queue_claimed = true;
    result.provider_delivery_receipt_requirement_status =
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_REQUIRED
            .to_string();
    result.provider_delivery_receipt_requirement_claimed = true;
    result.provider_delivery_receipt_pending_status =
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_PENDING
            .to_string();
    result.provider_delivery_receipt_pending_claimed = true;
    result.provider_delivery_receipt_ingested_status =
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_PENDING
            .to_string();
    result.provider_delivery_receipt_ingested_claimed = false;
}
