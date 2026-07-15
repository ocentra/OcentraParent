use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::AppGameTimerParentPreferenceSetupRequest;
use ocentra_parent_agent_protocol::AppGameTimerParentPreferenceSetupRequestResult;

const REQUEST_JSON: &str = r#"{"requestId":"app-game-timer-parent-preference-setup-request-1","requestedAt":"2026-06-08T10:45:00Z","parentSurfaceIntentReferenceId":"app-game-parent-surface-intent-1","parentPreferenceSetupReferenceId":"app-game-parent-preference-setup-1","requestReferenceIds":["app-game-parent-preference-setup-request-ref-1"]}"#;
const RESULT_JSON: &str = r#"{"schemaVersion":"app-game-timer-parent-preference-setup-request-proof","requestId":"app-game-timer-parent-preference-setup-request-1","requestedAt":"2026-06-08T10:45:00Z","acceptedAt":"2026-06-08T10:45:01Z","requestStatus":"accepted","parentSurfaceIntentReferenceId":"app-game-parent-surface-intent-1","parentPreferenceSetupReferenceId":"app-game-parent-preference-setup-1","requestReferenceIds":["app-game-parent-preference-setup-request-ref-1"],"actionResultReferenceId":"app-game-parent-action-result-1","actionResultReferenceIds":["app-game-parent-action-result-ref-1"],"actionResultPersistenceStatus":"persisted","parentPreferenceMutationReceiptId":"app-game-parent-preference-receipt-1","parentPreferenceMutationReceiptIds":["app-game-parent-preference-receipt-ref-1"],"parentPreferenceMutationReceiptStatus":"persisted","parentPreferenceMutationReceiptClaimed":false,"childRuntimeDeliveryHandoffId":"app-game-child-runtime-delivery-handoff-1","childRuntimeDeliveryHandoffIds":["app-game-child-runtime-delivery-handoff-ref-1"],"childRuntimeDeliveryHandoffStatus":"handoff-ready","childRuntimeDeliveryHandoffClaimed":false,"childRuntimeDeliveryQueueId":"app-game-child-runtime-delivery-queue-1","childRuntimeDeliveryQueueIds":["app-game-child-runtime-delivery-queue-ref-1"],"childRuntimeDeliveryQueueStatus":"queued","childRuntimeDeliveryQueueClaimed":false,"childRuntimeDeliveryDispatchId":"app-game-child-runtime-delivery-dispatch-1","childRuntimeDeliveryDispatchIds":["app-game-child-runtime-delivery-dispatch-ref-1"],"childRuntimeDeliveryDispatchStatus":"dispatch-ready","childRuntimeDeliveryDispatchClaimed":false,"childRuntimeDeliveryReceiptRequirementId":"app-game-child-runtime-delivery-receipt-requirement-1","childRuntimeDeliveryReceiptRequirementIds":["app-game-child-runtime-delivery-receipt-requirement-ref-1"],"childRuntimeDeliveryReceiptRequirementStatus":"receipt-required","childRuntimeDeliveryReceiptRequirementClaimed":false,"childRuntimeDeliveryReceiptPendingId":"app-game-child-runtime-delivery-receipt-pending-1","childRuntimeDeliveryReceiptPendingIds":["app-game-child-runtime-delivery-receipt-pending-ref-1"],"childRuntimeDeliveryReceiptPendingStatus":"receipt-pending","childRuntimeDeliveryReceiptPendingClaimed":false,"childRuntimeDeliveryReceiptIngestedId":"app-game-child-runtime-delivery-receipt-ingested-1","childRuntimeDeliveryReceiptIngestedIds":["app-game-child-runtime-delivery-receipt-ingested-ref-1"],"childRuntimeDeliveryReceiptIngestedStatus":"receipt-ingested","childRuntimeDeliveryReceiptIngestedClaimed":false,"durableOutboxRecordId":"app-game-parent-durable-outbox-record-1","durableOutboxRecordIds":["app-game-parent-durable-outbox-record-ref-1"],"durableOutboxStatus":"outbox-recorded","providerDeliveryReadinessId":"app-game-provider-delivery-readiness-1","providerDeliveryReadinessIds":["app-game-provider-delivery-readiness-ref-1"],"providerDeliveryReadinessStatus":"provider-manual-required","providerDeliveryAttemptId":"app-game-provider-delivery-attempt-1","providerDeliveryAttemptIds":["app-game-provider-delivery-attempt-ref-1"],"providerDeliveryAttemptStatus":"provider-delivery-manual-required","providerDeliveryAdapterRequirementId":"app-game-provider-delivery-adapter-requirement-1","providerDeliveryAdapterRequirementIds":["app-game-provider-delivery-adapter-requirement-ref-1"],"providerDeliveryAdapterRequirementStatus":"provider-adapter-required","providerDeliveryCredentialRequirementId":"app-game-provider-delivery-credential-requirement-1","providerDeliveryCredentialRequirementIds":["app-game-provider-delivery-credential-requirement-ref-1"],"providerDeliveryCredentialRequirementStatus":"provider-credential-proof-required","providerDeliveryQueueId":"app-game-provider-delivery-queue-1","providerDeliveryQueueIds":["app-game-provider-delivery-queue-ref-1"],"providerDeliveryQueueStatus":"provider-delivery-queued","providerDeliveryReceiptRequirementId":"app-game-provider-delivery-receipt-requirement-1","providerDeliveryReceiptRequirementIds":["app-game-provider-delivery-receipt-requirement-ref-1"],"providerDeliveryReceiptRequirementStatus":"provider-delivery-receipt-required","providerDeliveryReceiptPendingId":"app-game-provider-delivery-receipt-pending-1","providerDeliveryReceiptPendingIds":["app-game-provider-delivery-receipt-pending-ref-1"],"providerDeliveryReceiptPendingStatus":"provider-delivery-receipt-pending","providerDeliveryReceiptIngestedId":"app-game-provider-delivery-receipt-ingested-1","providerDeliveryReceiptIngestedIds":["app-game-provider-delivery-receipt-ingested-ref-1"],"providerDeliveryReceiptIngestedStatus":"provider-delivery-receipt-ingested","commandBoundaryClaimed":true,"actionResultHandoffClaimed":true,"actionResultPersistenceClaimed":false,"parentPreferenceMutationClaimed":false,"notificationRuleMutationClaimed":false,"providerDeliveryReadinessClaimed":false,"providerDeliveryAttemptClaimed":false,"providerDeliveryAdapterRequirementClaimed":false,"providerDeliveryCredentialRequirementClaimed":false,"providerDeliveryQueueClaimed":false,"providerDeliveryReceiptRequirementClaimed":false,"providerDeliveryReceiptPendingClaimed":false,"providerDeliveryReceiptIngestedClaimed":false,"providerDeliveryClaimed":false,"providerReceiptIngestionClaimed":false,"childRuntimeDeliveryClaimed":false,"durableOutboxClaimed":false,"adapterDispatchClaimed":false,"broadBlockingClaimed":false,"platformEnforcementClaimed":false,"rawPrivateSourceRowsClaimed":false,"rawTargetValuesClaimed":false,"privateDiagnosticsClaimed":false}"#;

#[test]
fn app_game_timer_parent_preference_setup_request_round_trips_through_json() {
    let request = serde_json::from_str::<AppGameTimerParentPreferenceSetupRequest>(REQUEST_JSON)
        .expect_value("request parses");
    let reparsed = serde_json::from_value::<AppGameTimerParentPreferenceSetupRequest>(
        serde_json::to_value(&request).expect_value("request serializes"),
    )
    .expect_value("request reparses");

    assert_eq!(
        reparsed.request_id,
        "app-game-timer-parent-preference-setup-request-1"
    );
    assert_eq!(
        reparsed.request_reference_ids,
        request.request_reference_ids
    );
}

#[test]
fn app_game_timer_parent_preference_setup_request_result_round_trips_through_json() {
    let result =
        serde_json::from_str::<AppGameTimerParentPreferenceSetupRequestResult>(RESULT_JSON)
            .expect_value("request result parses");
    let reparsed = serde_json::from_value::<AppGameTimerParentPreferenceSetupRequestResult>(
        serde_json::to_value(&result).expect_value("request result serializes"),
    )
    .expect_value("request result reparses");

    assert_eq!(
        reparsed.schema_version,
        "app-game-timer-parent-preference-setup-request-proof"
    );
    assert!(!reparsed.provider_delivery_claimed);
    assert!(!reparsed.adapter_dispatch_claimed);
}
