use std::collections::BTreeSet;

use ocentra_schema::tracking_event_contracts::{
    tracking_event_contract, validate_tracking_event_contract, TrackingEventContractError,
    TrackingEventContractInput, TrackingEventFamily, TRACKING_EVENT_CONTRACTS,
};

#[test]
fn tracking_event_contracts_are_unique_and_cover_every_wp34_family() {
    let event_types = TRACKING_EVENT_CONTRACTS
        .iter()
        .map(|specification| specification.event_type)
        .collect::<BTreeSet<_>>();
    assert_eq!(event_types.len(), TRACKING_EVENT_CONTRACTS.len());
    for event_type in [
        "tracking.config.change-requested",
        "tracking.config.change-approved",
        "tracking.config.change-rejected",
        "tracking.config.applied",
        "location.evidence.observed",
        "geofence.transition.evaluated",
        "expected-place.status.evaluated",
        "nearby-place.analysis.requested",
        "nearby-place.analysis.completed",
        "tracking.detection.completed",
        "tracking.live-mode.start-requested",
        "tracking.live-mode.started",
        "tracking.live-mode.stop-requested",
        "tracking.live-mode.stopped",
        "notification.intent.created",
        "notification.dispatch.requested",
        "notification.dispatch.result-observed",
        "escalation.intent.created",
        "escalation.result-observed",
    ] {
        assert!(event_types.contains(event_type), "missing {event_type}");
    }
}

#[test]
fn tracking_event_contracts_reject_missing_causation_evidence_and_policy_refs() {
    let input = valid_input();
    assert_eq!(
        validate_tracking_event_contract(
            "geofence.transition.evaluated",
            TrackingEventContractInput {
                causation_id: None,
                ..input
            }
        ),
        Err(TrackingEventContractError::MissingCausationId)
    );
    assert_eq!(
        validate_tracking_event_contract(
            "location.evidence.observed",
            TrackingEventContractInput {
                evidence_ref: None,
                ..input
            }
        ),
        Err(TrackingEventContractError::MissingEvidenceRef)
    );
    assert_eq!(
        validate_tracking_event_contract(
            "notification.dispatch.requested",
            TrackingEventContractInput {
                policy_ref: None,
                ..input
            }
        ),
        Err(TrackingEventContractError::MissingPolicyRef)
    );
}

#[test]
fn tracking_event_contracts_bind_live_mode_and_ai_safety_fields() {
    let input = valid_input();
    assert_eq!(
        validate_tracking_event_contract(
            "tracking.live-mode.started",
            TrackingEventContractInput {
                live_mode_ttl_seconds: Some(0),
                ..input
            }
        ),
        Err(TrackingEventContractError::MissingLiveModeTtl)
    );
    assert_eq!(
        validate_tracking_event_contract(
            "nearby-place.analysis.requested",
            TrackingEventContractInput {
                ai_authority_field_present: true,
                ..input
            }
        ),
        Err(TrackingEventContractError::AiAuthorityFieldForbidden)
    );
    assert_eq!(
        tracking_event_contract("nearby-place.analysis.completed")
            .map(|specification| specification.family),
        Some(TrackingEventFamily::Ai)
    );
}

fn valid_input() -> TrackingEventContractInput<'static> {
    TrackingEventContractInput {
        event_id: Some("event-1"),
        correlation_id: Some("correlation-1"),
        causation_id: Some("cause-1"),
        aggregate_key: Some("household:child-device"),
        evidence_ref: Some("evidence-1"),
        policy_ref: Some("policy-1"),
        idempotency_key: Some("idempotency-1"),
        audit_ref: Some("audit-1"),
        uncertainty_state: Some("ambiguous"),
        live_mode_ttl_seconds: Some(60),
        ai_authority_field_present: false,
    }
}
