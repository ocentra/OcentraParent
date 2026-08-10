use super::super::super::*;

const SCREEN_SETTINGS_STATUS_ACCEPTED: &str = "accepted";

pub(crate) struct PayloadText(pub(crate) String);

macro_rules! payload_text {
    ($value:expr) => {
        PayloadText($value.to_string())
    };
}

pub(crate) fn screen_settings_response_event(
    request_id: PayloadText,
    kind: PayloadText,
    event: AgentEventName,
    status: PayloadText,
    rejection_reason: Option<PayloadText>,
) -> AgentEventEnvelope {
    let PayloadText(request_id) = request_id;
    let PayloadText(kind) = kind;
    let PayloadText(status) = status;
    let rejection_reason = rejection_reason.map(|PayloadText(value)| value);
    let mut payload = std::collections::BTreeMap::new();
    payload.insert(
        constants::field::SCREEN_SETTINGS_RESPONSE.to_string(),
        LogFieldValue::String(require_ok(
            serde_json::to_string(&json!({
                "schemaVersion": 1,
                "requestId": request_id.as_str(),
                "kind": kind.as_str(),
                "status": status.as_str(),
                "setting": Value::Null,
                "auditEventId": Value::Null,
                "rejectionReason": rejection_reason.as_deref(),
                "message": if status == SCREEN_SETTINGS_STATUS_ACCEPTED {
                    "Screen settings state reported."
                } else {
                    "Screen settings update rejected."
                }
            })),
            "screen settings response serializes",
        )),
    );
    payload.insert(
        constants::field::SCREEN_SETTINGS_UPDATE_KIND.to_string(),
        LogFieldValue::String(kind.clone()),
    );
    if let Some(rejection_reason) = rejection_reason.as_deref() {
        payload.insert(
            constants::field::SCREEN_SETTINGS_REJECTION_REASON.to_string(),
            LogFieldValue::String(rejection_reason.to_string()),
        );
    }

    AgentEventEnvelope {
        schema_version: 1,
        event_id: format!("screen-settings-{kind}-{status}"),
        correlation_id: request_id,
        sent_at: "2026-06-23T00:00:03Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            role: AgentPeerRole::AgentService,
        },
        target: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        event,
        severity: LogLevel::Info,
        payload: payload.into(),
        snapshot: None,
    }
}

pub(crate) fn network_runtime_event_chain_response_event() -> AgentEventEnvelope {
    let entries = json!([
        {
            "eventType": constants::network_flow::EVENT_AI_ANALYSIS_COMPLETED,
            "payload": {
                "aiAnalysisRef": "event.ai.analysis.completed.1"
            }
        },
        {
            "eventType": constants::network_flow::EVENT_POLICY_DECISION_COMPLETED,
            "payload": {
                "policyDecisionRef": "event.policy.decision.completed.1"
            }
        },
        {
            "eventType": constants::network_flow::EVENT_ENFORCEMENT_RESULT_OBSERVED,
            "payload": {
                "enforcementResultRef": "event.enforcement.result.observed.1"
            }
        }
    ]);
    let mut payload = std::collections::BTreeMap::new();
    payload.insert(
        constants::field::NETWORK_RUNTIME_STREAMED_EVENTS.to_string(),
        LogFieldValue::Number(3.0),
    );
    payload.insert(
        constants::field::NETWORK_RUNTIME_EVENT_CHAIN_STREAM.to_string(),
        LogFieldValue::String(require_ok(
            serde_json::to_string(&entries),
            "network runtime event chain serializes",
        )),
    );

    AgentEventEnvelope {
        schema_version: 1,
        event_id: "agent.network.runtime.event-chain.reported-1".to_string(),
        correlation_id: "network-runtime".to_string(),
        sent_at: "2026-06-23T00:00:01Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            role: AgentPeerRole::AgentService,
        },
        target: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        event: AgentEventName::AgentNetworkRuntimeEventChainStreamReported,
        severity: LogLevel::Info,
        payload: payload.into(),
        snapshot: None,
    }
}

pub(crate) fn tracking_retention_settings_write_response_event() -> AgentEventEnvelope {
    let mut payload = std::collections::BTreeMap::new();
    payload.insert(
        constants::field::ACTIVITY_TRACKING_RETENTION_SETTINGS_WRITE_RESULT.to_string(),
        LogFieldValue::String(require_ok(
            serde_json::to_string(&json!({
                "schemaVersion": 1,
                "commandId": "tracking-retention-settings-write-command",
                "settingsKind": "retention-window-setting",
                "writeState": "service-write-command-accepted",
                "acceptedAt": "2026-06-06T19:40:00.000Z",
                "sourceWriterIntentRefs": ["tracking-retention-settings-write-retention-window"],
                "sourceReadModelProofRefs": [
                    "output/tracking-plan-proof/07-retention-and-custody-model/18-retention-settings-read-model-proof.json"
                ],
                "sourceMutationProofRefs": [
                    "output/tracking-plan-proof/07-retention-and-custody-model/20-retention-settings-mutation-proof.json"
                ],
                "appliedRetentionWindowHours": 168,
                "appliedDeleteAfterAlertResolutionState": "retain-after-alert-resolved",
                "parentExportState": "not-prepared",
                "remoteSyncState": "disabled",
                "remoteAiState": "disabled",
                "localServiceStateRevision": 1,
                "localServiceStateSnapshotRef": "agent-service-local-retention-settings-state",
                "durableSettingsStoreRef": "agent-service-local-retention-settings-durable-json",
                "durableSettingsPersistenceState": "persisted",
                "commandTransportClaimState": "claimed",
                "serviceWritePreflightClaimState": "claimed",
                "serviceMutationExecutionState": "claimed",
                "portalWritableUiClaimState": "unclaimed",
                "platformRuntimeClaimState": "unclaimed",
                "childDeviceDeliveryClaimState": "unclaimed",
                "providerDeliveryClaimState": "unclaimed",
                "notificationReceiptClaimState": "unclaimed",
                "physicalDeviceClaimState": "unclaimed",
                "authorityClaimState": "unclaimed",
                "productClaimState": "unclaimed"
            })),
            "tracking retention settings write result serializes",
        )),
    );

    AgentEventEnvelope {
        schema_version: 1,
        event_id: "tracking-retention-settings-write-result".to_string(),
        correlation_id: "tracking-retention-settings-write-command".to_string(),
        sent_at: "2026-06-06T19:40:01Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            role: AgentPeerRole::AgentService,
        },
        target: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        event: AgentEventName::AgentActivityTrackingRetentionSettingsWriteReported,
        severity: LogLevel::Info,
        payload: payload.into(),
        snapshot: None,
    }
}

pub(crate) fn app_game_adapter_dispatch_execute_response_event(
    command_id: PayloadText,
) -> AgentEventEnvelope {
    let PayloadText(command_id) = command_id;
    let mut payload = std::collections::BTreeMap::new();
    payload.insert(
        constants::field::APP_GAME_ADAPTER_DISPATCH_EXECUTE_RESULT.to_string(),
        LogFieldValue::String(require_ok(
            serde_json::to_string(&json!({
                "schemaVersion": 1,
                "commandId": command_id.as_str(),
                "generatedAt": "2026-06-08T12:45:00Z",
                "sourceReadModelId": "app-game-adapter-dispatch-result",
                "sourceDispatchRowId": "app-game-adapter-dispatch-result-windows-app-game-owned-process-time-limit",
                "sourceProofEntryId": "windows-app-game-owned-process-time-limit",
                "executionCommandName": "agent.enforcement.execute",
                "executionEventName": "agent.enforcement.audit.reported",
                "executionResultId": "enforcement-result-app-game-owned-process",
                "executionStatus": "actually-enforced",
                "executionAdapterResultCode": "process-already-exited",
                "executionAuditEventId": "enforcement-audit-app-game-owned-process",
                "readbackCommandName": "agent.activity.app-game.adapter-dispatch-result.read-model.get",
                "adapterDispatchExecutedClaimed": true,
                "broadInstalledAppBlockingClaimed": false,
                "childDeviceDeliveryClaimed": false,
                "platformEnforcementClaimed": false,
                "providerDeliveryClaimed": false,
                "privateDiagnosticsClaimed": false
            })),
            "app-game adapter dispatch execute result serializes",
        )),
    );

    AgentEventEnvelope {
        schema_version: 1,
        event_id: "evt-app-game-dispatch-executed-latest".to_string(),
        correlation_id: command_id,
        sent_at: "2026-06-08T12:45:01Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            role: AgentPeerRole::AgentService,
        },
        target: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        event: AgentEventName::AgentActivityAppGameAdapterDispatchExecuted,
        severity: LogLevel::Info,
        payload: payload.into(),
        snapshot: None,
    }
}

pub(crate) fn app_game_timer_parent_preference_setup_requested_response_event() -> AgentEventEnvelope
{
    let mut payload = std::collections::BTreeMap::new();
    payload.insert(
        constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUEST.to_string(),
        LogFieldValue::String(require_ok(
            serde_json::to_string(&app_game_timer_parent_preference_setup_result_payload()),
            "app-game timer parent preference setup result serializes",
        )),
    );

    AgentEventEnvelope {
        schema_version: 1,
        event_id: "app-game-parent-preference-setup-requested".to_string(),
        correlation_id: "request-1".to_string(),
        sent_at: "2026-06-08T02:18:01Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            role: AgentPeerRole::AgentService,
        },
        target: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        event: AgentEventName::AgentActivityAppGameTimerParentPreferenceSetupRequested,
        severity: LogLevel::Info,
        payload: payload.into(),
        snapshot: None,
    }
}

fn app_game_timer_parent_preference_setup_result_payload() -> Value {
    let provider_readiness = "app-game-parent-preference-setup-provider-readiness::request-1";
    let provider_attempt = "app-game-parent-preference-setup-provider-attempt::request-1";
    let provider_adapter = "app-game-parent-preference-setup-provider-adapter::request-1";
    let provider_credential = "app-game-parent-preference-setup-provider-credential::request-1";
    let provider_receipt_required =
        "app-game-parent-preference-setup-provider-receipt-required::request-1";
    let provider_receipt_pending =
        "app-game-parent-preference-setup-provider-receipt-pending::request-1";
    let provider_receipt_ingested =
        "app-game-parent-preference-setup-provider-receipt-ingested::request-1";

    json!({
        "schemaVersion": "app-game-timer-parent-preference-setup-request-proof",
        "requestId": "request-1",
        "requestedAt": "2026-06-08T02:18:00Z",
        "acceptedAt": "2026-06-08T02:18:01Z",
        "requestStatus": "accepted",
        "parentSurfaceIntentReferenceId": "app-game-child-ux-parent-surface-action-result-app-game-1",
        "parentPreferenceSetupReferenceId": "app-game-child-ux-parent-preference-setup-action-result-app-game-1",
        "requestReferenceIds": [
            "app-game-child-ux-local-handoff-action-result-app-game-1",
            "parent-approved",
            "child-status-limit-reached"
        ],
        "actionResultReferenceId": "app-game-parent-preference-setup-action-result::request-1",
        "actionResultReferenceIds": ["app-game-parent-preference-setup-action-result::request-1"],
        "actionResultPersistenceStatus": "persisted",
        "parentPreferenceMutationReceiptId": "app-game-parent-preference-setup-mutation-receipt::request-1",
        "parentPreferenceMutationReceiptIds": ["app-game-parent-preference-setup-mutation-receipt::request-1"],
        "parentPreferenceMutationReceiptStatus": "persisted",
        "parentPreferenceMutationReceiptClaimed": false,
        "childRuntimeDeliveryHandoffId": "app-game-parent-preference-setup-child-runtime-handoff::request-1",
        "childRuntimeDeliveryHandoffIds": ["app-game-parent-preference-setup-child-runtime-handoff::request-1"],
        "childRuntimeDeliveryHandoffStatus": "handoff-ready",
        "childRuntimeDeliveryHandoffClaimed": false,
        "childRuntimeDeliveryQueueId": "app-game-parent-preference-setup-child-runtime-queue::request-1",
        "childRuntimeDeliveryQueueIds": ["app-game-parent-preference-setup-child-runtime-queue::request-1"],
        "childRuntimeDeliveryQueueStatus": "queued",
        "childRuntimeDeliveryQueueClaimed": false,
        "childRuntimeDeliveryDispatchId": "app-game-parent-preference-setup-child-runtime-dispatch::request-1",
        "childRuntimeDeliveryDispatchIds": ["app-game-parent-preference-setup-child-runtime-dispatch::request-1"],
        "childRuntimeDeliveryDispatchStatus": "dispatch-ready",
        "childRuntimeDeliveryDispatchClaimed": false,
        "childRuntimeDeliveryReceiptRequirementId": "app-game-parent-preference-setup-child-runtime-receipt-required::request-1",
        "childRuntimeDeliveryReceiptRequirementIds": ["app-game-parent-preference-setup-child-runtime-receipt-required::request-1"],
        "childRuntimeDeliveryReceiptRequirementStatus": "receipt-required",
        "childRuntimeDeliveryReceiptRequirementClaimed": false,
        "childRuntimeDeliveryReceiptPendingId": "app-game-parent-preference-setup-child-runtime-receipt-pending::request-1",
        "childRuntimeDeliveryReceiptPendingIds": ["app-game-parent-preference-setup-child-runtime-receipt-pending::request-1"],
        "childRuntimeDeliveryReceiptPendingStatus": "receipt-pending",
        "childRuntimeDeliveryReceiptPendingClaimed": false,
        "childRuntimeDeliveryReceiptIngestedId": "app-game-parent-preference-setup-child-runtime-receipt-ingested::request-1",
        "childRuntimeDeliveryReceiptIngestedIds": ["app-game-parent-preference-setup-child-runtime-receipt-ingested::request-1"],
        "childRuntimeDeliveryReceiptIngestedStatus": "receipt-ingested",
        "childRuntimeDeliveryReceiptIngestedClaimed": false,
        "durableOutboxRecordId": "app-game-parent-preference-setup-outbox::request-1",
        "durableOutboxRecordIds": ["app-game-parent-preference-setup-outbox::request-1"],
        "durableOutboxStatus": "outbox-recorded",
        "providerDeliveryReadinessId": provider_readiness,
        "providerDeliveryReadinessIds": [provider_readiness],
        "providerDeliveryReadinessStatus": "provider-manual-required",
        "providerDeliveryAttemptId": provider_attempt,
        "providerDeliveryAttemptIds": [provider_attempt],
        "providerDeliveryAttemptStatus": "provider-delivery-manual-required",
        "providerDeliveryAdapterRequirementId": provider_adapter,
        "providerDeliveryAdapterRequirementIds": [provider_adapter],
        "providerDeliveryAdapterRequirementStatus": "provider-adapter-required",
        "providerDeliveryCredentialRequirementId": provider_credential,
        "providerDeliveryCredentialRequirementIds": [provider_credential],
        "providerDeliveryCredentialRequirementStatus": "provider-credential-proof-required",
        "providerDeliveryQueueId": "app-game-parent-preference-setup-provider-queue::request-1",
        "providerDeliveryQueueIds": ["app-game-parent-preference-setup-provider-queue::request-1"],
        "providerDeliveryQueueStatus": "provider-delivery-queued",
        "providerDeliveryReceiptRequirementId": provider_receipt_required,
        "providerDeliveryReceiptRequirementIds": [provider_receipt_required],
        "providerDeliveryReceiptRequirementStatus": "provider-delivery-receipt-required",
        "providerDeliveryReceiptPendingId": provider_receipt_pending,
        "providerDeliveryReceiptPendingIds": [provider_receipt_pending],
        "providerDeliveryReceiptPendingStatus": "provider-delivery-receipt-pending",
        "providerDeliveryReceiptIngestedId": provider_receipt_ingested,
        "providerDeliveryReceiptIngestedIds": [provider_receipt_ingested],
        "providerDeliveryReceiptIngestedStatus": "provider-delivery-receipt-ingested",
        "commandBoundaryClaimed": true,
        "actionResultHandoffClaimed": true,
        "actionResultPersistenceClaimed": true,
        "parentPreferenceMutationClaimed": false,
        "notificationRuleMutationClaimed": false,
        "providerDeliveryReadinessClaimed": false,
        "providerDeliveryAttemptClaimed": false,
        "providerDeliveryAdapterRequirementClaimed": false,
        "providerDeliveryCredentialRequirementClaimed": false,
        "providerDeliveryQueueClaimed": false,
        "providerDeliveryReceiptRequirementClaimed": false,
        "providerDeliveryReceiptPendingClaimed": false,
        "providerDeliveryReceiptIngestedClaimed": false,
        "providerDeliveryClaimed": false,
        "providerReceiptIngestionClaimed": false,
        "childRuntimeDeliveryClaimed": false,
        "durableOutboxClaimed": true,
        "adapterDispatchClaimed": false,
        "broadBlockingClaimed": false,
        "platformEnforcementClaimed": false,
        "rawPrivateSourceRowsClaimed": false,
        "rawTargetValuesClaimed": false,
        "privateDiagnosticsClaimed": false
    })
}

pub(crate) fn policy_request_assistant_preview_confirmed_response_event() -> AgentEventEnvelope {
    let mut payload = std::collections::BTreeMap::new();
    payload.insert(
        constants::field::POLICY_REQUEST_ASSISTANT_PREVIEW_CONFIRM_RESULT.to_string(),
        LogFieldValue::String(require_ok(
            serde_json::to_string(&json!({
                "schemaVersion": 1,
                "commandId": "cmd-policy-request-confirm-1",
                "requestId": "policy-request-1",
                "assistantPreviewId": "policy-preview.network.1",
                "resultState": "confirmed",
                "policyRequestStatus": "pending-parent-review",
                "policyAssistantConfirmationState": "parent-confirmed",
                "policyAuditReferenceId": "audit.policy-request.confirmed",
                "confirmedAt": "2026-06-18T00:10:00Z",
                "rejectionReason": null,
                "commandTransportClaimState": "claimed",
                "serviceValidationClaimState": "claimed",
                "activityStoreMutationClaimState": "claimed",
                "upstreamWriterClaimState": "claimed",
                "readModelProjectionClaimState": "claimed",
                "portalWritableUiClaimState": "unclaimed",
                "childDeviceDeliveryClaimState": "unclaimed",
                "providerDeliveryClaimState": "unclaimed",
                "platformEnforcementClaimState": "unclaimed",
                "productClaimState": "unclaimed"
            })),
            "policy preview confirm result serializes",
        )),
    );

    AgentEventEnvelope {
        schema_version: 1,
        event_id: "agent.policy.request.assistant-preview.confirm.reported-1".to_string(),
        correlation_id: "cmd-policy-request-confirm-1".to_string(),
        sent_at: "2026-06-18T00:10:01Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            role: AgentPeerRole::AgentService,
        },
        target: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        event: AgentEventName::AgentPolicyRequestAssistantPreviewConfirmReported,
        severity: LogLevel::Info,
        payload: payload.into(),
        snapshot: None,
    }
}

pub(crate) fn policy_request_parent_resolution_resolved_response_event() -> AgentEventEnvelope {
    AgentEventEnvelope {
        schema_version: 1,
        event_id: "agent.policy.request.parent-resolution.resolved-1".to_string(),
        correlation_id: "cmd-policy-request-resolution-1".to_string(),
        sent_at: "2026-06-18T00:11:01Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            role: AgentPeerRole::AgentService,
        },
        target: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        event: AgentEventName::AgentPolicyRequestParentResolutionResolved,
        severity: LogLevel::Info,
        payload: std::collections::BTreeMap::new().into(),
        snapshot: None,
    }
}

pub(crate) fn app_game_route_load_response_events() -> Vec<AgentEventEnvelope> {
    vec![
        app_game_notification_readiness_response_event(),
        app_game_policy_readiness_response_event(),
        app_game_platform_proof_status_response_event(),
        app_game_child_runtime_transport_receipt_response_event(),
        app_game_adapter_dispatch_preflight_response_event(),
        app_game_adapter_dispatch_result_response_event(),
        app_game_timer_parent_surface_response_event(),
    ]
}

pub(crate) fn app_game_notification_readiness_response_event() -> AgentEventEnvelope {
    let read_model = AppGameNotificationReadinessReadModel {
        schema_version: 1,
        generated_at: "2026-06-08T12:45:03Z".to_string(),
        custody_label: "child-device-query-store".to_string(),
        capability_status: "notification-intent-ready".to_string(),
        returned: 1,
        ready_intent_count: 1,
        manual_required_count: 0,
        unavailable_count: 0,
        provider_delivery_claimed: false,
        provider_receipt_ingestion_claimed: false,
        local_outbox_runtime_claimed: false,
        scheduler_runtime_claimed: false,
        adapter_dispatch_claimed: false,
        parent_ui_claimed: true,
        child_delivery_claimed: false,
        rows: vec![AppGameNotificationReadinessRow {
            schema_version: 1,
            row_id: "notification-ready-row".to_string(),
            reason: "time-limit-exceeded".to_string(),
            readiness_state: "ready-for-local-intent".to_string(),
            row_count: 1,
            minimal_payload_ref: "minimal-alert:time-limit-exceeded".to_string(),
            evidence_reference_ids: vec!["event.notification.intent.1".to_string()],
            evidence: vec![ActivityEvidenceRef {
                evidence_id: "event.notification.intent.1".to_string(),
                kind: ActivityEvidenceKind::LocalDbRow,
                digest: None,
                uri: None,
            }],
        }],
    };
    app_game_read_model_response_event(
        payload_text!("app-game-notification-readiness-1"),
        payload_text!("app-game-notification"),
        AgentEventName::AgentActivityAppGameNotificationReadinessReadModelReported,
        payload_text!(constants::field::APP_GAME_NOTIFICATION_READINESS_READ_MODEL),
        &read_model,
    )
}

pub(crate) fn app_game_policy_readiness_response_event() -> AgentEventEnvelope {
    let read_model = AppGamePolicyReadinessReadModel {
        schema_version: 1,
        generated_at: "2026-06-08T12:45:04Z".to_string(),
        custody_label: "child-device-query-store".to_string(),
        capability_status: "policy-ready".to_string(),
        returned: 1,
        policy_evaluation_ready: true,
        category_routing_ready: true,
        unknown_review_required: false,
        manual_review_required: false,
        adapter_dispatch_claimed: false,
        evidence_claim_row_count: 1,
        identity_row_count: 0,
        approval_authority_row_count: 0,
        approval_action_result_row_count: 0,
        platform_authority_row_count: 0,
        ai_classifier_result_row_count: 0,
        category_candidate_row_count: 0,
        unknown_review_row_count: 0,
        rows: vec![AppGamePolicyReadinessRow {
            schema_version: 1,
            row_id: "policy-readiness-row-1".to_string(),
            readiness_kind: "policyEvidence".to_string(),
            readiness_state: "ready".to_string(),
            row_count: 1,
            evidence_reference_ids: vec!["event.policy.evidence.1".to_string()],
            evidence: vec![ActivityEvidenceRef {
                evidence_id: "event.policy.evidence.1".to_string(),
                kind: ActivityEvidenceKind::LocalDbRow,
                digest: None,
                uri: None,
            }],
        }],
    };
    app_game_read_model_response_event(
        payload_text!("app-game-policy-readiness-1"),
        payload_text!("app-game-policy"),
        AgentEventName::AgentActivityAppGamePolicyReadinessReadModelReported,
        payload_text!(constants::field::APP_GAME_POLICY_READINESS_READ_MODEL),
        &read_model,
    )
}

pub(crate) fn app_game_platform_proof_status_response_event() -> AgentEventEnvelope {
    let read_model = AppGamePlatformProofStatusReadModel {
        schema_version: 1,
        read_model_id: "app-game-platform-proof-status".to_string(),
        generated_at: "2026-06-08T12:45:05Z".to_string(),
        source_read_model_ids: vec!["app-game-platform-proof-status-source".to_string()],
        custody_label: "app-game-platform-proof-status".to_string(),
        capability_status: "app-game-platform-proof-status-partial".to_string(),
        returned: 1,
        host_visible_count: 1,
        host_not_detected_count: 0,
        local_runtime_not_applicable_count: 0,
        enforcement_ready_count: 1,
        open_gap_count: 1,
        adapter_dispatch_claimed: false,
        broad_installed_app_blocking_claimed: false,
        platform_enforcement_claimed: false,
        provider_delivery_claimed: false,
        child_device_delivery_claimed: false,
        private_diagnostics_claimed: false,
        rows: vec![AppGamePlatformProofStatusRow {
            schema_version: 1,
            row_id: "platform-proof-row-1".to_string(),
            platform: "Windows".to_string(),
            proof_state: "scoped-windows-execution-proved".to_string(),
            authority_state: "scoped-execution-only".to_string(),
            host_capability_state: "host-visible".to_string(),
            host_capability_evidence_refs: vec!["event.platform.windows.host-visible".to_string()],
            host_capability_probe_refs: vec!["probe.platform.windows".to_string()],
            product_meanings: vec!["native-app".to_string()],
            proof_refs: vec!["event.platform.windows.proof.1".to_string()],
            open_gaps: vec!["broad-installed-app-blocking-not-proved".to_string()],
            adapter_dispatch_claimed: false,
            broad_installed_app_blocking_claimed: false,
            platform_enforcement_claimed: false,
            provider_delivery_claimed: false,
            child_device_delivery_claimed: false,
            private_diagnostics_claimed: false,
            last_checked_at: "2026-06-08T12:45:05Z".to_string(),
        }],
    };
    app_game_read_model_response_event(
        payload_text!("app-game-platform-proof-status-1"),
        payload_text!("app-game-platform"),
        AgentEventName::AgentActivityAppGamePlatformProofStatusReadModelReported,
        payload_text!(constants::field::APP_GAME_PLATFORM_PROOF_STATUS_READ_MODEL),
        &read_model,
    )
}

pub(crate) fn app_game_child_runtime_transport_receipt_response_event() -> AgentEventEnvelope {
    let read_model = AppGameChildRuntimeTransportReceiptReadModel {
        schema_version: 1,
        read_model_id: "app-game-child-runtime-transport-receipt".to_string(),
        generated_at: "2026-06-08T12:45:06Z".to_string(),
        source_read_model_ids: vec!["app-game-child-runtime-transport-receipt-source".to_string()],
        custody_label: "app-game-child-runtime-transport-receipt".to_string(),
        capability_status: "app-game-child-runtime-transport-required".to_string(),
        returned: 1,
        transport_required_count: 1,
        manual_required_count: 0,
        unavailable_count: 0,
        runtime_transport_executed: false,
        runtime_receipt_ingested: false,
        provider_delivery_executed: false,
        platform_delivery_channel_claimed: false,
        adapter_dispatch_claimed: false,
        platform_enforcement_claimed: false,
        raw_private_source_rows_included: false,
        rows: vec![AppGameChildRuntimeTransportReceiptRow {
            schema_version: 1,
            row_id: "transport-receipt-row-1".to_string(),
            source_runtime_writer_row_id: "runtime-writer-row-1".to_string(),
            boundary_state: "child-runtime-transport-required".to_string(),
            product_meanings: vec!["native-game".to_string()],
            required_transport_refs: vec!["child-runtime-transport-contract-ref".to_string()],
            required_receipt_refs: vec!["child-runtime-delivery-receipt-contract-ref".to_string()],
            open_gaps: vec!["child-runtime-transport-not-executed".to_string()],
            runtime_transport_executed: false,
            runtime_receipt_ingested: false,
            provider_delivery_executed: false,
            platform_delivery_channel_claimed: false,
        }],
    };
    app_game_read_model_response_event(
        payload_text!("app-game-child-runtime-transport-receipt-1"),
        payload_text!("app-game-child-runtime-transport"),
        AgentEventName::AgentActivityAppGameChildRuntimeTransportReceiptReadModelReported,
        payload_text!(constants::field::APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL),
        &read_model,
    )
}

pub(crate) fn app_game_adapter_dispatch_preflight_response_event() -> AgentEventEnvelope {
    let read_model = AppGameAdapterDispatchPreflightReadModel {
        schema_version: 1,
        read_model_id: "app-game-adapter-dispatch-preflight".to_string(),
        generated_at: "2026-06-08T12:45:07Z".to_string(),
        source_read_model_ids: vec!["app-game-adapter-execution-readiness".to_string()],
        custody_label: "adapter-execution-readiness-and-policy-dispatch".to_string(),
        capability_status: "app-game-adapter-dispatch-preflight-partial".to_string(),
        returned: 0,
        dispatch_eligible_count: 0,
        blocked_before_dispatch_count: 0,
        adapter_dispatch_eligible_count: 0,
        adapter_dispatch_executed_claimed_count: 0,
        host_capability_available_count: 0,
        host_capability_not_detected_count: 0,
        host_capability_not_applicable_count: 0,
        host_capability_probe_ref_count: 0,
        broad_installed_app_blocking_claimed: false,
        child_device_delivery_claimed: false,
        platform_enforcement_claimed: false,
        provider_delivery_claimed: false,
        private_diagnostics_claimed: false,
        rows: Vec::new(),
    };
    app_game_read_model_response_event(
        payload_text!("app-game-adapter-dispatch-preflight-1"),
        payload_text!("app-game-adapter-dispatch-preflight"),
        AgentEventName::AgentActivityAppGameAdapterDispatchPreflightReadModelReported,
        payload_text!(constants::field::APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_READ_MODEL),
        &read_model,
    )
}

pub(crate) fn app_game_adapter_dispatch_result_response_event() -> AgentEventEnvelope {
    let read_model = AppGameAdapterDispatchResultReadModel {
        schema_version: 1,
        read_model_id: "app-game-adapter-dispatch-result".to_string(),
        generated_at: "2026-06-08T12:45:08Z".to_string(),
        source_read_model_ids: vec!["app-game-adapter-dispatch-preflight".to_string()],
        custody_label: "adapter-dispatch-preflight-and-enforcement-command-result".to_string(),
        capability_status: "app-game-adapter-dispatch-command-result-partial".to_string(),
        returned: 0,
        command_accepted_count: 0,
        blocked_before_command_count: 0,
        execution_audit_recorded_count: 0,
        blocked_before_execution_audit_count: 0,
        adapter_execution_reported_count: 0,
        adapter_execution_evidence_missing_count: 0,
        blocked_before_adapter_execution_count: 0,
        adapter_dispatch_command_result_claimed_count: 0,
        service_local_execution_audit_claimed_count: 0,
        adapter_dispatch_executed_claimed_count: 0,
        broad_installed_app_blocking_claimed: false,
        child_device_delivery_claimed: false,
        platform_enforcement_claimed: false,
        provider_delivery_claimed: false,
        private_diagnostics_claimed: false,
        rows: Vec::new(),
    };
    app_game_read_model_response_event(
        payload_text!("app-game-adapter-dispatch-result-1"),
        payload_text!("app-game-adapter-dispatch-result"),
        AgentEventName::AgentActivityAppGameAdapterDispatchResultReadModelReported,
        payload_text!(constants::field::APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL),
        &read_model,
    )
}

pub(crate) fn app_game_timer_parent_surface_response_event() -> AgentEventEnvelope {
    let read_model = AppGameTimerParentSurfaceReadModel {
        schema_version: 1,
        generated_at: "2026-06-08T12:45:09Z".to_string(),
        custody_label: "child-device-query-store".to_string(),
        capability_status: "timer-parent-surface-no-rows".to_string(),
        returned: 0,
        ready_for_parent_surface_count: 0,
        blocked_by_source_freshness_count: 0,
        blocked_by_compiler_decision_count: 0,
        runtime_manual_required_count: 0,
        control_action_result_count: 0,
        control_action_result_reference_ids: Vec::new(),
        control_action_result_statuses: Vec::new(),
        control_action_result_capability_states: Vec::new(),
        control_action_result_enforcement_statuses: Vec::new(),
        child_facing_reason_reference_ids: Vec::new(),
        child_facing_status_reference_ids: Vec::new(),
        child_ux_handoff_ready_count: 0,
        child_ux_handoff_blocked_count: 0,
        child_ux_handoff_reference_ids: Vec::new(),
        child_ux_local_handoff_artifact_record_count: 0,
        child_ux_local_handoff_artifact_skipped_count: 0,
        child_ux_local_handoff_artifact_reference_ids: Vec::new(),
        child_ux_local_handoff_artifact_records: Vec::new(),
        child_ux_parent_surface_intent_manual_action_required_count: 0,
        child_ux_parent_surface_intent_unavailable_visible_count: 0,
        child_ux_parent_surface_intent_history_visible_count: 0,
        child_ux_parent_surface_intent_preference_setup_required_count: 0,
        child_ux_parent_surface_intent_reference_ids: Vec::new(),
        child_ux_parent_surface_intent_records: Vec::new(),
        child_ux_parent_preference_setup_draft_ready_count: 0,
        child_ux_parent_preference_setup_unavailable_visible_count: 0,
        child_ux_parent_preference_setup_reference_ids: Vec::new(),
        child_ux_parent_preference_setup_request_ready_count: 0,
        child_ux_parent_preference_setup_request_unavailable_visible_count: 0,
        child_ux_parent_preference_setup_request_reference_ids: Vec::new(),
        child_ux_parent_preference_setup_records: Vec::new(),
        timer_runtime_claimed: false,
        scheduler_persistence_claimed: false,
        durable_scheduler_storage_claimed: false,
        audit_runtime_claimed: false,
        rollback_runtime_claimed: false,
        adapter_dispatch_claimed: false,
        child_delivery_claimed: false,
        platform_enforcement_claimed: false,
        raw_private_source_rows_included: false,
        rows: Vec::new(),
    };
    app_game_read_model_response_event(
        payload_text!("app-game-timer-parent-surface-1"),
        payload_text!("app-game-timer-parent-surface"),
        AgentEventName::AgentActivityAppGameTimerParentSurfaceReadModelReported,
        payload_text!(constants::field::APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL),
        &read_model,
    )
}

pub(crate) fn app_game_read_model_response_event(
    event_id: PayloadText,
    correlation_id: PayloadText,
    event: AgentEventName,
    field_name: PayloadText,
    read_model: &impl serde::Serialize,
) -> AgentEventEnvelope {
    let PayloadText(event_id) = event_id;
    let PayloadText(correlation_id) = correlation_id;
    let PayloadText(field_name) = field_name;
    let mut payload = std::collections::BTreeMap::new();
    payload.insert(
        field_name,
        LogFieldValue::String(require_ok(
            serde_json::to_string(read_model),
            "app-game read model serializes",
        )),
    );

    AgentEventEnvelope {
        schema_version: 1,
        event_id,
        correlation_id,
        sent_at: "2026-06-08T12:45:07Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            role: AgentPeerRole::AgentService,
        },
        target: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        event,
        severity: LogLevel::Info,
        payload: payload.into(),
        snapshot: None,
    }
}

macro_rules! policy_preview_response_payload {
    () => {{
        let mut payload = std::collections::BTreeMap::new();
        payload.insert(
            constants::field::SCHEMA_VERSION.to_string(),
            LogFieldValue::String("1".to_string()),
        );
        payload.insert(
            constants::field::GENERATED_AT.to_string(),
            LogFieldValue::String("2026-05-21T02:00:03Z".to_string()),
        );
        payload.insert(
            constants::field::CUSTODY.to_string(),
            LogFieldValue::String("child-device-query-store".to_string()),
        );
        payload.insert(
            constants::field::LIMIT.to_string(),
            LogFieldValue::Number(10.0),
        );
        payload.insert(
            constants::field::RETURNED.to_string(),
            LogFieldValue::Number(1.0),
        );
        payload.insert(
            constants::field::CAPABILITY_STATUS.to_string(),
            LogFieldValue::String("preview-ready".to_string()),
        );
        payload.insert(
            constants::field::POLICY_PREVIEW_ID.to_string(),
            LogFieldValue::String("policy-preview.network.1".to_string()),
        );
        payload.insert(
            constants::field::LATEST_EVENT_ID.to_string(),
            LogFieldValue::String("policy-preview.network.event.1".to_string()),
        );
        payload.insert(
            constants::field::LATEST_OBSERVED_AT.to_string(),
            LogFieldValue::String("2026-05-21T02:00:02Z".to_string()),
        );
        payload.insert(
            constants::field::POLICY_TARGET_TYPE.to_string(),
            LogFieldValue::String("network-domain".to_string()),
        );
        payload.insert(
            constants::field::POLICY_TARGET_VALUE.to_string(),
            LogFieldValue::String("example.test".to_string()),
        );
        payload.insert(
            constants::field::POLICY_DECISION_ID.to_string(),
            LogFieldValue::String("policy-decision.network.preview.1".to_string()),
        );
        payload.insert(
            constants::field::POLICY_ACTION.to_string(),
            LogFieldValue::String("block".to_string()),
        );
        payload.insert(
            constants::field::LOCAL_AI_RESULT_ID.to_string(),
            LogFieldValue::String("local-ai-result.network.preview.1".to_string()),
        );
        payload.insert(
            constants::field::POLICY_DRY_RUN.to_string(),
            LogFieldValue::Boolean(true),
        );
        payload.insert(
            constants::field::POLICY_HANDOFF_STATE.to_string(),
            LogFieldValue::String("disabled-preview-only".to_string()),
        );
        payload.insert(
            constants::field::NETWORK_EVIDENCE_GRADE.to_string(),
            LogFieldValue::String("A".to_string()),
        );
        payload.insert(
            constants::field::NETWORK_REQUESTED_POLICY_ACTION.to_string(),
            LogFieldValue::String("block".to_string()),
        );
        payload.insert(
            constants::field::NETWORK_MAPPED_POLICY_ACTION.to_string(),
            LogFieldValue::String("block".to_string()),
        );
        payload.insert(
            constants::field::NETWORK_POLICY_MAPPING_MODE.to_string(),
            LogFieldValue::String("dry-run".to_string()),
        );
        payload.insert(
            constants::field::NETWORK_ADAPTER_ACTION_AUTHORIZED.to_string(),
            LogFieldValue::Boolean(false),
        );
        payload.insert(
            constants::field::NETWORK_ENFORCEMENT_COMMAND_AUTHORIZED.to_string(),
            LogFieldValue::Boolean(false),
        );
        payload
    }};
}

pub(crate) fn policy_preview_response_event() -> AgentEventEnvelope {
    let payload = policy_preview_response_payload!();
    AgentEventEnvelope {
        schema_version: 1,
        event_id: "agent.policy.preview.read-model.reported-1".to_string(),
        correlation_id: "policy-preview".to_string(),
        sent_at: "2026-05-21T02:00:03Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            role: AgentPeerRole::AgentService,
        },
        target: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        event: AgentEventName::AgentPolicyPreviewReadModelReported,
        severity: LogLevel::Info,
        payload: payload.into(),
        snapshot: None,
    }
}

pub(crate) fn policy_preview_confirmed_response_event() -> AgentEventEnvelope {
    let mut event = policy_preview_response_event();
    event.payload.insert(
        constants::field::POLICY_REQUEST_ORIGIN.to_string(),
        LogFieldValue::String("assistant-draft".to_string()),
    );
    event.payload.insert(
        constants::field::POLICY_ASSISTANT_CONFIRMATION_STATE.to_string(),
        LogFieldValue::String("parent-confirmed".to_string()),
    );
    event.payload.insert(
        constants::field::POLICY_REQUEST_STATUS.to_string(),
        LogFieldValue::String("pending-parent-review".to_string()),
    );
    event.payload.insert(
        constants::field::POLICY_REVIEWED_BY_ACTOR_ID.to_string(),
        LogFieldValue::String("parent-1".to_string()),
    );
    event.payload.insert(
        constants::field::POLICY_REVIEWED_BY_ACTOR_ROLE.to_string(),
        LogFieldValue::String("parent".to_string()),
    );
    event.payload.insert(
        constants::field::POLICY_REVIEWED_AT.to_string(),
        LogFieldValue::String("2026-06-18T00:10:00Z".to_string()),
    );
    event.payload.insert(
        constants::field::POLICY_AUDIT_REFERENCE_ID.to_string(),
        LogFieldValue::String("audit.policy-request.confirmed".to_string()),
    );
    event
}
