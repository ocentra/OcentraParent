use crate::test_text::{test_ok as ok, test_some as some, TestResult, TestText};
use ocentra_eventing::{
    delivery::validation::EventDeliveryDecisionState,
    delivery::validation::EventDeliveryRequiredArtifact,
    delivery::validation::EventDeliveryRouteKind, topology::EventTopologyStatus,
};
use ocentra_parent_agent_core::browser_event_runtime::action_handoff::{
    browser_runtime_action_intent_handoff_topology_manifest,
    request_browser_runtime_action_intent_handoff_for_input,
};
use ocentra_parent_agent_core::browser_event_runtime::action_handoff_durable::prove_browser_runtime_action_intent_durable_handoff;
use ocentra_parent_agent_core::browser_event_runtime::action_handoff_durable_types::BrowserRuntimeActionIntentDurableHandoffReadModelState;
use ocentra_parent_agent_core::browser_event_runtime::action_status::{
    browser_runtime_action_intent_status_topology_manifest,
    request_browser_runtime_action_intent_status_for_input,
};
use ocentra_parent_agent_core::browser_event_runtime::delivery::prove_browser_runtime_delivery_decision;
use ocentra_parent_agent_core::browser_event_runtime::topology::browser_runtime_chain_topology_manifest;
use ocentra_parent_agent_core::browser_event_runtime::{
    publish_browser_runtime_chain_for_input, BrowserRuntimeEventPayload, BrowserRuntimeInput,
    BrowserRuntimeReport,
};
use ocentra_parent_agent_protocol::browser::BrowserRuntimePhase;
use ocentra_parent_agent_protocol::constants;

#[path = "browser_event_runtime_tests/browser_event_runtime_child_status_tests.rs"]
mod browser_event_runtime_child_status_tests;
#[path = "browser_event_runtime_tests/browser_event_runtime_parent_surface_tests.rs"]
mod browser_event_runtime_parent_surface_tests;
#[path = "browser_event_runtime_tests/browser_event_runtime_social_provider_receipt_tests.rs"]
mod browser_event_runtime_social_provider_receipt_tests;
#[path = "browser_event_runtime_tests/browser_event_runtime_stream_report_tests.rs"]
mod browser_event_runtime_stream_report_tests;

#[tokio::test]
async fn browser_runtime_chain_publishes_ordered_managed_decision_phases() -> TestResult {
    let report = ok(
        publish_browser_runtime_chain_for_input(BrowserRuntimeInput::managed_decision_fixture())
            .await,
        "publish managed decision fixture",
    )?;

    assert_eq!(report.dead_letters.len(), 0);
    assert_eq!(
        report.publish_reports.len(),
        BrowserRuntimePhase::ordered_chain().len()
    );
    assert_eq!(
        decoded_phases(&report)?,
        BrowserRuntimePhase::ordered_chain().to_vec()
    );
    assert_previous_refs_follow_published_events(&report)?;
    assert!(report.intervention_command_published());
    assert_all_payloads_preserve_browser_context(
        &report,
        constants::browser::CAPABILITY_STATUS_AVAILABLE,
        constants::browser::CUSTODY_CHILD_DEVICE_LOCAL,
        constants::browser::QUERY_VISIBILITY_LIVE_LOCAL,
        None::<TestText>,
    )?;
    Ok(())
}

#[tokio::test]
async fn browser_runtime_chain_keeps_manual_required_rows_non_executing() -> TestResult {
    let report = ok(
        publish_browser_runtime_chain_for_input(BrowserRuntimeInput::manual_required_fixture())
            .await,
        "publish manual required fixture",
    )?;

    assert_eq!(decoded_phases(&report)?, non_executing_runtime_phases());
    assert!(!report.intervention_command_published());
    assert_previous_refs_follow_published_events(&report)?;
    assert_all_payloads_preserve_browser_context(
        &report,
        constants::browser::CAPABILITY_STATUS_AVAILABLE,
        constants::browser::CUSTODY_CHILD_DEVICE_LOCAL,
        constants::browser::QUERY_VISIBILITY_LIVE_LOCAL,
        None::<TestText>,
    )?;

    let policy_event = some(
        decoded_payloads(&report)?
            .into_iter()
            .find(|payload| payload.phase == BrowserRuntimePhase::PolicyDecisionCompleted),
        "policy decision payload missing",
    )?;
    assert!(!policy_event.ai_authority);
    assert!(!policy_event.policy_authority);
    assert!(!policy_event.intervention_command_allowed);
    assert!(!policy_event.adapter_dispatch_claimed);
    Ok(())
}

#[tokio::test]
async fn browser_runtime_chain_carries_dry_run_action_handoff_without_dispatch() -> TestResult {
    let report = ok(
        publish_browser_runtime_chain_for_input(
            BrowserRuntimeInput::dry_run_action_handoff_fixture(),
        )
        .await,
        "publish dry run action handoff fixture",
    )?;

    assert_eq!(decoded_phases(&report)?, non_executing_runtime_phases());
    assert!(!report.intervention_command_published());
    assert_previous_refs_follow_published_events(&report)?;

    let policy_event = some(
        decoded_payloads(&report)?
            .into_iter()
            .find(|payload| payload.phase == BrowserRuntimePhase::PolicyDecisionCompleted),
        "policy decision payload missing",
    )?;
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
    Ok(())
}

#[tokio::test]
async fn browser_runtime_action_intent_handoff_prepares_outbox_without_dispatch() -> TestResult {
    let report = ok(
        publish_browser_runtime_chain_for_input(
            BrowserRuntimeInput::dry_run_action_handoff_fixture(),
        )
        .await,
        "publish dry run action handoff fixture",
    )?;

    assert_eq!(decoded_phases(&report)?, non_executing_runtime_phases());
    assert!(!report.intervention_command_published());

    let (candidate_count, policy_preview_id, action_intent_id, event_ref, outbox_ref, handoff_ref) =
        some(
            report.action_intent_handoff_summary(),
            "action intent handoff summary missing",
        )?;
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
    Ok(())
}

#[tokio::test]
async fn browser_runtime_action_intent_handoff_event_subscriber_prepares_outbox_without_dispatch(
) -> TestResult {
    let report = ok(
        request_browser_runtime_action_intent_handoff_for_input(
            BrowserRuntimeInput::dry_run_action_handoff_fixture(),
        )
        .await,
        "request action intent handoff",
    )?;

    assert_eq!(report.dead_letters.len(), 0);
    assert_eq!(report.request_report.publish_report.handled_count, 1);
    assert_eq!(
        some(
            report.stored_events.first(),
            "handoff request event missing"
        )?
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
        .ok_or_else(|| TestText::from_display("handoff source event ref missing"))?
        .ends_with(constants::browser::EVENT_BROWSER_POLICY_DECISION_COMPLETED));
    assert_eq!(handoff.dispatch_attempt_count, 0);
    assert_eq!(handoff.adapter_execution_count, 0);
    assert_eq!(handoff.browser_mutation_count, 0);
    assert_eq!(handoff.child_intervention_execution_count, 0);
    assert_eq!(handoff.enforcement_execution_count, 0);
    assert!(handoff.dry_run_only);
    assert!(handoff.policy_authority_only);
    Ok(())
}

#[tokio::test]
async fn browser_runtime_action_intent_handoff_event_subscriber_keeps_manual_rows_empty(
) -> TestResult {
    let report = ok(
        request_browser_runtime_action_intent_handoff_for_input(
            BrowserRuntimeInput::manual_required_fixture(),
        )
        .await,
        "request manual action handoff",
    )?;

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
    Ok(())
}

#[tokio::test]
async fn browser_runtime_action_intent_durable_handoff_preserves_refs_without_execution(
) -> TestResult {
    let report = ok(
        prove_browser_runtime_action_intent_durable_handoff().await,
        "prove browser runtime durable handoff",
    )?;

    assert_eq!(report.request_event_count, 1);
    assert_eq!(report.durable_record_count, 1);
    assert_eq!(report.read_model_row_count, 1);
    assert_eq!(report.prepared_not_dispatched_count, 1);
    assert!(report.duplicate_request_event_rejected);
    assert!(report.row_matches_handoff_response);
    assert!(report.row_matches_request_event);

    let row = some(report.rows.first(), "durable handoff row missing")?;
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
    Ok(())
}

#[tokio::test]
async fn browser_runtime_action_intent_event_subscriber_returns_pending_status() -> TestResult {
    let report = ok(
        request_browser_runtime_action_intent_status_for_input(
            BrowserRuntimeInput::dry_run_action_handoff_fixture(),
        )
        .await,
        "request action intent status",
    )?;

    assert_eq!(report.dead_letters.len(), 0);
    assert_eq!(report.request_report.publish_report.handled_count, 1);
    assert_eq!(
        some(report.stored_events.first(), "status request event missing")?
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
    Ok(())
}

#[tokio::test]
async fn browser_runtime_action_intent_event_subscriber_keeps_manual_rows_empty() -> TestResult {
    let report = ok(
        request_browser_runtime_action_intent_status_for_input(
            BrowserRuntimeInput::manual_required_fixture(),
        )
        .await,
        "request manual action status",
    )?;

    let status = report.request_report.response;
    assert_eq!(status.candidate_count, 0);
    assert_eq!(status.policy_preview_id, None);
    assert_eq!(status.action_intent_id, None);
    assert_eq!(status.dispatch_attempt_count, 0);
    assert_eq!(status.adapter_execution_count, 0);
    assert_eq!(status.child_intervention_execution_count, 0);
    assert_eq!(status.enforcement_execution_count, 0);
    Ok(())
}

#[test]
fn browser_runtime_action_intent_handoff_topology_covers_named_event_and_subscriber() -> TestResult
{
    let manifest = ok(
        browser_runtime_action_intent_handoff_topology_manifest(),
        "browser action intent handoff topology",
    )?;
    assert_eq!(manifest.unready_entries().len(), 0);
    assert_eq!(manifest.entries().len(), 1);

    let entry = some(
        manifest.entries().first(),
        "browser action intent handoff topology entry missing",
    )?;
    assert_eq!(entry.status, EventTopologyStatus::Covered);
    assert_eq!(
        entry.contract.event_type.as_str(),
        constants::browser::EVENT_BROWSER_ACTION_INTENT_HANDOFF_REQUESTED
    );
    assert_eq!(
        some(
            entry.publishers.first(),
            "browser action intent handoff publisher missing",
        )?
        .as_str(),
        constants::browser::RUNTIME_COMPONENT_BROWSER_SPINE
    );
    let subscriber = some(
        entry.subscribers.first(),
        "browser action intent handoff subscriber missing",
    )?;
    assert_eq!(
        subscriber.subscriber_id.as_str(),
        constants::browser::SUBSCRIBER_BROWSER_ACTION_INTENT_HANDOFF
    );
    assert_eq!(
        subscriber.target_handler.as_str(),
        constants::browser::TARGET_BROWSER_ACTION_INTENT_HANDOFF
    );
    Ok(())
}

#[test]
fn browser_runtime_action_intent_topology_covers_named_event_and_subscriber() -> TestResult {
    let manifest = ok(
        browser_runtime_action_intent_status_topology_manifest(),
        "browser action intent status topology",
    )?;
    assert_eq!(manifest.unready_entries().len(), 0);
    assert_eq!(manifest.entries().len(), 1);

    let entry = some(
        manifest.entries().first(),
        "browser action intent status topology entry missing",
    )?;
    assert_eq!(entry.status, EventTopologyStatus::Covered);
    assert_eq!(
        entry.contract.event_type.as_str(),
        constants::browser::EVENT_BROWSER_ACTION_INTENT_STATUS_REQUESTED
    );
    assert_eq!(
        some(
            entry.publishers.first(),
            "browser action intent status publisher missing",
        )?
        .as_str(),
        constants::browser::RUNTIME_COMPONENT_BROWSER_SPINE
    );
    let subscriber = some(
        entry.subscribers.first(),
        "browser action intent status subscriber missing",
    )?;
    assert_eq!(
        subscriber.subscriber_id.as_str(),
        constants::browser::SUBSCRIBER_BROWSER_ACTION_INTENT_STATUS
    );
    assert_eq!(
        subscriber.target_handler.as_str(),
        constants::browser::TARGET_BROWSER_ACTION_INTENT_STATUS
    );
    Ok(())
}

#[test]
fn browser_runtime_chain_topology_covers_ordered_event_spine() -> TestResult {
    let manifest = ok(
        browser_runtime_chain_topology_manifest(),
        "browser runtime chain topology",
    )?;
    assert_eq!(manifest.unready_entries().len(), 0);
    assert_eq!(
        manifest.entries().len(),
        BrowserRuntimePhase::ordered_chain().len()
    );

    for phase in BrowserRuntimePhase::ordered_chain() {
        let entry = some(
            manifest
                .entries()
                .iter()
                .find(|entry| entry.contract.event_type.as_str() == phase.event_type()),
            "browser runtime chain topology phase entry missing",
        )?;
        assert_eq!(entry.status, EventTopologyStatus::Covered);
        assert_eq!(
            some(
                entry.publishers.first(),
                "browser runtime chain topology publisher missing",
            )?
            .as_str(),
            constants::browser::RUNTIME_COMPONENT_BROWSER_SPINE
        );
        let subscriber = some(
            entry.subscribers.first(),
            "browser runtime chain topology subscriber missing",
        )?;
        assert_eq!(subscriber.subscriber_id.as_str(), phase.subscriber_id());
        assert_eq!(subscriber.target_handler.as_str(), phase.target_handler());
    }
    Ok(())
}

#[test]
fn browser_runtime_delivery_decision_keeps_current_routes_local_only() -> TestResult {
    let report = ok(
        prove_browser_runtime_delivery_decision(),
        "prove browser runtime delivery decision",
    )?;

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
        report.external_transport_delivery.missing_artifacts,
        vec![
            EventDeliveryRequiredArtifact::CustodyProof,
            EventDeliveryRequiredArtifact::PublisherAuthProof,
            EventDeliveryRequiredArtifact::SubscriberAuthProof,
            EventDeliveryRequiredArtifact::EncryptionProof,
            EventDeliveryRequiredArtifact::RetentionPolicy,
            EventDeliveryRequiredArtifact::ReplayPlan,
            EventDeliveryRequiredArtifact::DeletionPlan,
            EventDeliveryRequiredArtifact::OffsetPolicy,
            EventDeliveryRequiredArtifact::DedupePolicy,
            EventDeliveryRequiredArtifact::TransportConfig,
        ]
    );
    assert!(report.external_transport_manual_required);
    assert!(!report.external_transport_delivery_implemented);
    assert!(!report.external_relay_delivery_implemented);
    assert!(!report.adapter_dispatch_claimed);
    assert!(!report.browser_mutation_claimed);
    assert!(!report.child_intervention_execution_claimed);
    assert!(!report.final_policy_execution_claimed);
    assert!(!report.enforcement_claimed);
    Ok(())
}

fn assert_all_payloads_preserve_browser_context<D: std::fmt::Display>(
    report: &BrowserRuntimeReport,
    capability_status: impl std::fmt::Display,
    custody_label: impl std::fmt::Display,
    query_visibility: impl std::fmt::Display,
    degraded_reason: Option<D>,
) -> TestResult {
    let capability_status = capability_status.to_string();
    let custody_label = custody_label.to_string();
    let query_visibility = query_visibility.to_string();
    let degraded_reason = degraded_reason.map(|reason| reason.to_string());
    for payload in decoded_payloads(report)? {
        assert_eq!(payload.capability_status, capability_status);
        assert_eq!(payload.custody_label, custody_label);
        assert_eq!(payload.query_visibility, query_visibility);
        assert_eq!(
            payload.degraded_reason.as_deref(),
            degraded_reason.as_deref()
        );
    }
    Ok(())
}

fn assert_previous_refs_follow_published_events(report: &BrowserRuntimeReport) -> TestResult {
    let decoded = decoded_payloads(report)?;
    assert_eq!(
        some(decoded.first(), "browser runtime first payload missing")?.previous_phase_ref,
        None
    );

    for pair in decoded.windows(2) {
        let previous = &pair[0];
        let current = &pair[1];
        assert_eq!(
            current.previous_phase_ref,
            Some(event_ref(previous).to_string())
        );
    }
    Ok(())
}

fn non_executing_runtime_phases() -> Vec<BrowserRuntimePhase> {
    BrowserRuntimePhase::ordered_chain()
        .iter()
        .cloned()
        .filter(|phase| {
            !matches!(
                phase,
                BrowserRuntimePhase::InterventionCommandIssued
                    | BrowserRuntimePhase::InterventionResultObserved
            )
        })
        .collect()
}

fn event_ref(payload: &BrowserRuntimeEventPayload) -> TestText {
    let mut value = String::from(constants::browser::CORRELATION_BROWSER_RUNTIME_PREFIX);
    value.push_str(&payload.evidence_ref);
    value.push(constants::delimiter::HYPHEN);
    value.push_str(&payload.observed_at);
    value.push(constants::delimiter::HYPHEN);
    value.push_str(payload.phase.event_type());
    TestText::from_display(value)
}

fn decoded_phases(report: &BrowserRuntimeReport) -> Result<Vec<BrowserRuntimePhase>, TestText> {
    Ok(decoded_payloads(report)?
        .into_iter()
        .map(|payload| payload.phase)
        .collect())
}

fn decoded_payloads(
    report: &BrowserRuntimeReport,
) -> Result<Vec<BrowserRuntimeEventPayload>, TestText> {
    report
        .stored_events
        .iter()
        .map(|event| {
            ok(
                event.decode::<BrowserRuntimeEventPayload>(),
                "decode browser runtime event payload",
            )
            .map(|decoded| decoded.payload)
        })
        .collect()
}
