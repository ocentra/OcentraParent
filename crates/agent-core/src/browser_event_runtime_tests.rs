use crate::browser_event_runtime::{
    prove_browser_runtime_action_intent_durable_handoff,
    BrowserRuntimeActionIntentDurableHandoffReadModelState,
};
use crate::{
    browser_runtime_action_intent_handoff_topology_manifest,
    browser_runtime_action_intent_status_topology_manifest,
    browser_runtime_chain_topology_manifest, prove_browser_runtime_delivery_decision,
    publish_browser_runtime_chain_for_input,
    request_browser_runtime_action_intent_handoff_for_input,
    request_browser_runtime_action_intent_status_for_input, BrowserRuntimeEventPayload,
    BrowserRuntimeInput, BrowserRuntimePhase, BrowserRuntimeReport,
};
use ocentra_eventing::{
    delivery::EventDeliveryDecisionState, delivery::EventDeliveryRequiredArtifact,
    delivery::EventDeliveryRouteKind, topology::EventTopologyStatus,
};
use ocentra_parent_agent_protocol::constants;

mod browser_event_runtime_child_status_tests;
mod browser_event_runtime_parent_surface_tests;
mod browser_event_runtime_social_provider_receipt_tests;
mod browser_event_runtime_stream_report_tests;

#[tokio::test]
async fn browser_runtime_chain_publishes_ordered_managed_decision_phases() {
    let report =
        publish_browser_runtime_chain_for_input(BrowserRuntimeInput::managed_decision_fixture())
            .await
            .unwrap();

    assert_eq!(report.dead_letters.len(), 0);
    assert_eq!(
        report.publish_reports.len(),
        BrowserRuntimePhase::ordered_chain().len()
    );
    assert_eq!(
        decoded_phases(&report),
        BrowserRuntimePhase::ordered_chain().to_vec()
    );
    assert_previous_refs_follow_published_events(&report);
    assert!(report.intervention_command_published());
    assert_all_payloads_preserve_browser_context(
        &report,
        constants::browser::CAPABILITY_STATUS_AVAILABLE,
        constants::browser::CUSTODY_CHILD_DEVICE_LOCAL,
        constants::browser::QUERY_VISIBILITY_LIVE_LOCAL,
        None,
    );
}

#[tokio::test]
async fn browser_runtime_chain_keeps_manual_required_rows_non_executing() {
    let report =
        publish_browser_runtime_chain_for_input(BrowserRuntimeInput::manual_required_fixture())
            .await
            .unwrap();

    let phases = decoded_phases(&report);
    assert!(phases.contains(&BrowserRuntimePhase::EvidenceObserved));
    assert!(phases.contains(&BrowserRuntimePhase::EvidenceJournaled));
    assert!(phases.contains(&BrowserRuntimePhase::AuditEntryCommitted));
    assert!(phases.contains(&BrowserRuntimePhase::ReadModelProjected));
    assert!(!phases.contains(&BrowserRuntimePhase::InterventionCommandIssued));
    assert!(!phases.contains(&BrowserRuntimePhase::InterventionResultObserved));
    assert!(!report.intervention_command_published());
    assert_previous_refs_follow_published_events(&report);
    assert_all_payloads_preserve_browser_context(
        &report,
        constants::browser::CAPABILITY_STATUS_AVAILABLE,
        constants::browser::CUSTODY_CHILD_DEVICE_LOCAL,
        constants::browser::QUERY_VISIBILITY_LIVE_LOCAL,
        None,
    );

    let policy_event = decoded_payloads(&report)
        .into_iter()
        .find(|payload| payload.phase == BrowserRuntimePhase::PolicyDecisionCompleted)
        .unwrap();
    assert!(!policy_event.ai_authority);
    assert!(!policy_event.policy_authority);
    assert!(!policy_event.intervention_command_allowed);
    assert!(!policy_event.adapter_dispatch_claimed);
}

#[tokio::test]
async fn browser_runtime_chain_carries_dry_run_action_handoff_without_dispatch() {
    let report = publish_browser_runtime_chain_for_input(
        BrowserRuntimeInput::dry_run_action_handoff_fixture(),
    )
    .await
    .unwrap();

    let phases = decoded_phases(&report);
    assert!(phases.contains(&BrowserRuntimePhase::PolicyEvaluationRequested));
    assert!(phases.contains(&BrowserRuntimePhase::PolicyDecisionCompleted));
    assert!(!phases.contains(&BrowserRuntimePhase::InterventionCommandIssued));
    assert!(!phases.contains(&BrowserRuntimePhase::InterventionResultObserved));
    assert!(!report.intervention_command_published());
    assert_previous_refs_follow_published_events(&report);

    let policy_event = decoded_payloads(&report)
        .into_iter()
        .find(|payload| payload.phase == BrowserRuntimePhase::PolicyDecisionCompleted)
        .unwrap();
    assert_eq!(
        policy_event.policy_preview_id.as_deref(),
        Some(constants::browser::TEST_BROWSER_RUNTIME_POLICY_PREVIEW_ID)
    );
    assert_eq!(
        policy_event.action_intent_id.as_deref(),
        Some(constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_ID)
    );
    assert!(policy_event.dry_run);
    assert!(policy_event.policy_authority);
    assert!(!policy_event.adapter_dispatch_claimed);
    assert!(!policy_event.intervention_command_allowed);
}

#[tokio::test]
async fn browser_runtime_action_intent_handoff_prepares_outbox_without_dispatch() {
    let report = publish_browser_runtime_chain_for_input(
        BrowserRuntimeInput::dry_run_action_handoff_fixture(),
    )
    .await
    .unwrap();

    assert!(!report.stored_events.is_empty());
    assert!(!report.intervention_command_published());

    let (candidate_count, policy_preview_id, action_intent_id, event_ref, outbox_ref, handoff_ref) =
        report.action_intent_handoff_summary().unwrap();
    assert_eq!(candidate_count, 1);
    assert_eq!(
        policy_preview_id,
        constants::browser::TEST_BROWSER_RUNTIME_POLICY_PREVIEW_ID
    );
    assert_eq!(
        action_intent_id,
        constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_ID
    );
    assert!(event_ref.ends_with(constants::browser::EVENT_BROWSER_POLICY_DECISION_COMPLETED));
    assert_eq!(
        outbox_ref,
        constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_OUTBOX_REF
    );
    assert_eq!(
        handoff_ref,
        constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_REF
    );
}

#[tokio::test]
async fn browser_runtime_action_intent_handoff_event_subscriber_prepares_outbox_without_dispatch() {
    let report = request_browser_runtime_action_intent_handoff_for_input(
        BrowserRuntimeInput::dry_run_action_handoff_fixture(),
    )
    .await
    .unwrap();

    assert_eq!(report.dead_letters.len(), 0);
    assert_eq!(report.request_report.publish_report.handled_count, 1);
    assert_eq!(
        report
            .stored_events
            .first()
            .unwrap()
            .contract
            .event_type
            .as_str(),
        constants::browser::EVENT_BROWSER_ACTION_INTENT_HANDOFF_REQUESTED
    );

    let handoff = report.request_report.response;
    assert_eq!(handoff.candidate_count, 1);
    assert_eq!(
        handoff.policy_preview_id.as_deref(),
        Some(constants::browser::TEST_BROWSER_RUNTIME_POLICY_PREVIEW_ID)
    );
    assert_eq!(
        handoff.action_intent_id.as_deref(),
        Some(constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_ID)
    );
    assert_eq!(
        handoff.outbox_ref.as_deref(),
        Some(constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_OUTBOX_REF)
    );
    assert_eq!(
        handoff.handoff_ref.as_deref(),
        Some(constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_REF)
    );
    assert!(handoff
        .source_event_ref
        .as_deref()
        .unwrap()
        .ends_with(constants::browser::EVENT_BROWSER_POLICY_DECISION_COMPLETED));
    assert_eq!(handoff.dispatch_attempt_count, 0);
    assert_eq!(handoff.adapter_execution_count, 0);
    assert_eq!(handoff.browser_mutation_count, 0);
    assert_eq!(handoff.child_intervention_execution_count, 0);
    assert_eq!(handoff.enforcement_execution_count, 0);
    assert!(handoff.dry_run_only);
    assert!(handoff.policy_authority_only);
}

#[tokio::test]
async fn browser_runtime_action_intent_handoff_event_subscriber_keeps_manual_rows_empty() {
    let report = request_browser_runtime_action_intent_handoff_for_input(
        BrowserRuntimeInput::manual_required_fixture(),
    )
    .await
    .unwrap();

    let handoff = report.request_report.response;
    assert_eq!(handoff.candidate_count, 0);
    assert_eq!(handoff.policy_preview_id, None);
    assert_eq!(handoff.action_intent_id, None);
    assert_eq!(handoff.source_event_ref, None);
    assert_eq!(handoff.outbox_ref, None);
    assert_eq!(handoff.handoff_ref, None);
    assert_eq!(handoff.dispatch_attempt_count, 0);
    assert_eq!(handoff.adapter_execution_count, 0);
    assert_eq!(handoff.browser_mutation_count, 0);
    assert_eq!(handoff.child_intervention_execution_count, 0);
    assert_eq!(handoff.enforcement_execution_count, 0);
}

#[tokio::test]
async fn browser_runtime_action_intent_durable_handoff_preserves_refs_without_execution() {
    let report = prove_browser_runtime_action_intent_durable_handoff()
        .await
        .unwrap();

    assert_eq!(report.request_event_count, 1);
    assert_eq!(report.durable_record_count, 1);
    assert_eq!(report.read_model_row_count, 1);
    assert_eq!(report.prepared_not_dispatched_count, 1);
    assert!(report.duplicate_request_event_rejected);
    assert!(report.row_matches_handoff_response);
    assert!(report.row_matches_request_event);

    let row = report.rows.first().unwrap();
    assert_eq!(
        row.state,
        BrowserRuntimeActionIntentDurableHandoffReadModelState::PreparedNotDispatched
    );
    assert_eq!(
        row.request_event_type.as_str(),
        constants::browser::EVENT_BROWSER_ACTION_INTENT_HANDOFF_REQUESTED
    );
    assert_eq!(
        row.policy_preview_id,
        constants::browser::TEST_BROWSER_RUNTIME_POLICY_PREVIEW_ID
    );
    assert_eq!(
        row.action_intent_id,
        constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_ID
    );
    assert_eq!(
        row.durable_result_ref.as_str(),
        constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_DURABLE_RESULT_REF
    );
    assert_eq!(
        row.durable_store_ref.as_str(),
        constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_DURABLE_STORE_REF
    );
    assert_eq!(
        row.outbox_ref.as_str(),
        constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_OUTBOX_REF
    );
    assert_eq!(
        row.handoff_ref.as_str(),
        constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_REF
    );
    assert_eq!(
        row.read_model_ref.as_str(),
        constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_READ_MODEL_REF
    );
    assert_eq!(
        row.support_status_ref.as_str(),
        constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_SUPPORT_STATUS_REF
    );
    assert_eq!(report.dispatch_attempt_count, 0);
    assert_eq!(report.adapter_execution_count, 0);
    assert_eq!(report.browser_mutation_count, 0);
    assert_eq!(report.child_intervention_execution_count, 0);
    assert_eq!(report.final_policy_execution_count, 0);
    assert_eq!(report.enforcement_execution_count, 0);
    assert!(!report.external_transport_implemented);
    assert!(!report.adapter_dispatch_claimed);
    assert!(!report.browser_mutation_claimed);
    assert!(!report.child_intervention_execution_claimed);
    assert!(!report.final_policy_execution_claimed);
    assert!(!report.enforcement_claimed);
}

#[tokio::test]
async fn browser_runtime_action_intent_event_subscriber_returns_pending_status() {
    let report = request_browser_runtime_action_intent_status_for_input(
        BrowserRuntimeInput::dry_run_action_handoff_fixture(),
    )
    .await
    .unwrap();

    assert_eq!(report.dead_letters.len(), 0);
    assert_eq!(report.request_report.publish_report.handled_count, 1);
    assert_eq!(
        report
            .stored_events
            .first()
            .unwrap()
            .contract
            .event_type
            .as_str(),
        constants::browser::EVENT_BROWSER_ACTION_INTENT_STATUS_REQUESTED
    );

    let status = report.request_report.response;
    assert_eq!(status.candidate_count, 1);
    assert_eq!(
        status.policy_preview_id.as_deref(),
        Some(constants::browser::TEST_BROWSER_RUNTIME_POLICY_PREVIEW_ID)
    );
    assert_eq!(
        status.action_intent_id.as_deref(),
        Some(constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_ID)
    );
    assert_eq!(status.dispatch_attempt_count, 0);
    assert_eq!(status.adapter_execution_count, 0);
    assert_eq!(status.child_intervention_execution_count, 0);
    assert_eq!(status.enforcement_execution_count, 0);
    assert!(status.dry_run_only);
    assert!(status.policy_authority_only);
}

#[tokio::test]
async fn browser_runtime_action_intent_event_subscriber_keeps_manual_rows_empty() {
    let report = request_browser_runtime_action_intent_status_for_input(
        BrowserRuntimeInput::manual_required_fixture(),
    )
    .await
    .unwrap();

    let status = report.request_report.response;
    assert_eq!(status.candidate_count, 0);
    assert_eq!(status.policy_preview_id, None);
    assert_eq!(status.action_intent_id, None);
    assert_eq!(status.dispatch_attempt_count, 0);
    assert_eq!(status.adapter_execution_count, 0);
    assert_eq!(status.child_intervention_execution_count, 0);
    assert_eq!(status.enforcement_execution_count, 0);
}

#[test]
fn browser_runtime_action_intent_handoff_topology_covers_named_event_and_subscriber() {
    let manifest = browser_runtime_action_intent_handoff_topology_manifest().unwrap();
    assert_eq!(manifest.unready_entries().len(), 0);
    assert_eq!(manifest.entries().len(), 1);

    let entry = manifest.entries().first().unwrap();
    assert_eq!(entry.status, EventTopologyStatus::Covered);
    assert_eq!(
        entry.contract.event_type.as_str(),
        constants::browser::EVENT_BROWSER_ACTION_INTENT_HANDOFF_REQUESTED
    );
    assert_eq!(
        entry.publishers.first().unwrap().as_str(),
        constants::browser::RUNTIME_COMPONENT_BROWSER_SPINE
    );
    let subscriber = entry.subscribers.first().unwrap();
    assert_eq!(
        subscriber.subscriber_id.as_str(),
        constants::browser::SUBSCRIBER_BROWSER_ACTION_INTENT_HANDOFF
    );
    assert_eq!(
        subscriber.target_handler.as_str(),
        constants::browser::TARGET_BROWSER_ACTION_INTENT_HANDOFF
    );
}

#[test]
fn browser_runtime_action_intent_topology_covers_named_event_and_subscriber() {
    let manifest = browser_runtime_action_intent_status_topology_manifest().unwrap();
    assert_eq!(manifest.unready_entries().len(), 0);
    assert_eq!(manifest.entries().len(), 1);

    let entry = manifest.entries().first().unwrap();
    assert_eq!(entry.status, EventTopologyStatus::Covered);
    assert_eq!(
        entry.contract.event_type.as_str(),
        constants::browser::EVENT_BROWSER_ACTION_INTENT_STATUS_REQUESTED
    );
    assert_eq!(
        entry.publishers.first().unwrap().as_str(),
        constants::browser::RUNTIME_COMPONENT_BROWSER_SPINE
    );
    let subscriber = entry.subscribers.first().unwrap();
    assert_eq!(
        subscriber.subscriber_id.as_str(),
        constants::browser::SUBSCRIBER_BROWSER_ACTION_INTENT_STATUS
    );
    assert_eq!(
        subscriber.target_handler.as_str(),
        constants::browser::TARGET_BROWSER_ACTION_INTENT_STATUS
    );
}

#[test]
fn browser_runtime_chain_topology_covers_ordered_event_spine() {
    let manifest = browser_runtime_chain_topology_manifest().unwrap();
    assert_eq!(manifest.unready_entries().len(), 0);
    assert_eq!(
        manifest.entries().len(),
        BrowserRuntimePhase::ordered_chain().len()
    );

    for phase in BrowserRuntimePhase::ordered_chain() {
        let entry = manifest
            .entries()
            .iter()
            .find(|entry| entry.contract.event_type.as_str() == phase.event_type())
            .unwrap();
        assert_eq!(entry.status, EventTopologyStatus::Covered);
        assert_eq!(
            entry.publishers.first().unwrap().as_str(),
            constants::browser::RUNTIME_COMPONENT_BROWSER_SPINE
        );
        let subscriber = entry.subscribers.first().unwrap();
        assert_eq!(subscriber.subscriber_id.as_str(), phase.subscriber_id());
        assert_eq!(subscriber.target_handler.as_str(), phase.target_handler());
    }
}

#[test]
fn browser_runtime_delivery_decision_keeps_current_routes_local_only() {
    let report = prove_browser_runtime_delivery_decision().unwrap();

    macro_rules! assert_ready_route {
        ($proof:expr, $kind:expr, $state:expr) => {{
            assert_eq!($proof.route_kind, $kind);
            assert_eq!($proof.decision_state, $state);
        }};
    }

    assert_eq!(report.local_ready_route_count, 8);
    assert_ready_route!(
        report.chain_delivery,
        EventDeliveryRouteKind::LocalService,
        EventDeliveryDecisionState::LocalRouteReady
    );
    for proof in [
        report.action_intent_status_delivery,
        report.action_intent_handoff_delivery,
        report.runtime_stream_report_delivery,
        report.social_provider_receipt_status_delivery,
        report.social_report_writer_delivery_status_delivery,
        report.social_parent_notification_delivery_status_delivery,
        report.social_parent_surface_status_delivery,
    ] {
        assert_ready_route!(
            proof,
            EventDeliveryRouteKind::LocalInProcess,
            EventDeliveryDecisionState::LocalRouteReady
        );
    }
    assert_ready_route!(
        report.external_transport_delivery,
        EventDeliveryRouteKind::ExternalTransport,
        EventDeliveryDecisionState::ExternalTransportRouteManualRequired
    );
    assert_eq!(
        report.external_transport_delivery.missing_artifacts.len(),
        10
    );
    assert!(report
        .external_transport_delivery
        .missing_artifacts
        .contains(&EventDeliveryRequiredArtifact::CustodyProof));
    assert!(report
        .external_transport_delivery
        .missing_artifacts
        .contains(&EventDeliveryRequiredArtifact::TransportConfig));
    assert!(report.external_transport_manual_required);
    assert!(!report.external_transport_delivery_implemented);
    assert!(!report.external_relay_delivery_implemented);
    assert!(!report.adapter_dispatch_claimed);
    assert!(!report.browser_mutation_claimed);
    assert!(!report.child_intervention_execution_claimed);
    assert!(!report.final_policy_execution_claimed);
    assert!(!report.enforcement_claimed);
}

fn assert_all_payloads_preserve_browser_context(
    report: &BrowserRuntimeReport,
    capability_status: &str,
    custody_label: &str,
    query_visibility: &str,
    degraded_reason: Option<&str>,
) {
    for payload in decoded_payloads(report) {
        assert_eq!(payload.capability_status, capability_status);
        assert_eq!(payload.custody_label, custody_label);
        assert_eq!(payload.query_visibility, query_visibility);
        assert_eq!(payload.degraded_reason.as_deref(), degraded_reason);
    }
}

fn assert_previous_refs_follow_published_events(report: &BrowserRuntimeReport) {
    let decoded = decoded_payloads(report);
    assert_eq!(decoded.first().unwrap().previous_phase_ref, None);

    for pair in decoded.windows(2) {
        let previous = &pair[0];
        let current = &pair[1];
        assert_eq!(current.previous_phase_ref, Some(event_ref(previous)));
    }
}

fn event_ref(payload: &BrowserRuntimeEventPayload) -> String {
    let mut value = String::from(constants::browser::CORRELATION_BROWSER_RUNTIME_PREFIX);
    value.push_str(&payload.evidence_ref);
    value.push(constants::delimiter::HYPHEN);
    value.push_str(&payload.observed_at);
    value.push(constants::delimiter::HYPHEN);
    value.push_str(payload.phase.event_type());
    value
}

fn decoded_phases(report: &BrowserRuntimeReport) -> Vec<BrowserRuntimePhase> {
    decoded_payloads(report)
        .into_iter()
        .map(|payload| payload.phase)
        .collect()
}

fn decoded_payloads(report: &BrowserRuntimeReport) -> Vec<BrowserRuntimeEventPayload> {
    report
        .stored_events
        .iter()
        .map(|event| {
            event
                .decode::<BrowserRuntimeEventPayload>()
                .unwrap()
                .payload
        })
        .collect()
}
