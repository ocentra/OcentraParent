//! Canonical tracking event names and cross-boundary payload requirements.
//!
//! Generic envelope, journal, replay, and idempotency mechanics remain in
//! `ocentra-eventing`; this module owns only tracking product event identity.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TrackingEventFamily {
    Config,
    Evidence,
    Evaluation,
    Ai,
    Detection,
    LiveMode,
    Notification,
    Escalation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrackingEventContractSpec {
    pub event_type: &'static str,
    pub family: TrackingEventFamily,
    pub requires_causation: bool,
    pub requires_evidence_ref: bool,
    pub requires_policy_ref: bool,
    pub requires_live_mode_ttl: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TrackingEventContractInput<'a> {
    pub event_id: Option<&'a str>,
    pub correlation_id: Option<&'a str>,
    pub causation_id: Option<&'a str>,
    pub aggregate_key: Option<&'a str>,
    pub evidence_ref: Option<&'a str>,
    pub policy_ref: Option<&'a str>,
    pub idempotency_key: Option<&'a str>,
    pub audit_ref: Option<&'a str>,
    pub uncertainty_state: Option<&'a str>,
    pub live_mode_ttl_seconds: Option<u64>,
    pub ai_authority_field_present: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrackingEventContractError {
    UnknownEventType,
    MissingEventId,
    MissingCorrelationId,
    MissingAggregateKey,
    MissingIdempotencyKey,
    MissingCausationId,
    MissingEvidenceRef,
    MissingPolicyRef,
    MissingAuditRef,
    MissingLiveModeTtl,
    MissingUncertaintyState,
    AiAuthorityFieldForbidden,
}

pub const TRACKING_EVENT_CONTRACTS: [TrackingEventContractSpec; 19] = [
    spec(
        "tracking.config.change-requested",
        TrackingEventFamily::Config,
        true,
        false,
        false,
        false,
    ),
    spec(
        "tracking.config.change-approved",
        TrackingEventFamily::Config,
        true,
        false,
        true,
        false,
    ),
    spec(
        "tracking.config.change-rejected",
        TrackingEventFamily::Config,
        true,
        false,
        true,
        false,
    ),
    spec(
        "tracking.config.applied",
        TrackingEventFamily::Config,
        true,
        false,
        true,
        false,
    ),
    spec(
        "location.evidence.observed",
        TrackingEventFamily::Evidence,
        false,
        true,
        false,
        false,
    ),
    spec(
        "geofence.transition.evaluated",
        TrackingEventFamily::Evaluation,
        true,
        true,
        false,
        false,
    ),
    spec(
        "expected-place.status.evaluated",
        TrackingEventFamily::Evaluation,
        true,
        true,
        true,
        false,
    ),
    spec(
        "nearby-place.analysis.requested",
        TrackingEventFamily::Ai,
        true,
        true,
        false,
        false,
    ),
    spec(
        "nearby-place.analysis.completed",
        TrackingEventFamily::Ai,
        true,
        true,
        false,
        false,
    ),
    spec(
        "tracking.detection.completed",
        TrackingEventFamily::Detection,
        true,
        true,
        true,
        false,
    ),
    spec(
        "tracking.live-mode.start-requested",
        TrackingEventFamily::LiveMode,
        true,
        true,
        true,
        true,
    ),
    spec(
        "tracking.live-mode.started",
        TrackingEventFamily::LiveMode,
        true,
        true,
        true,
        true,
    ),
    spec(
        "tracking.live-mode.stop-requested",
        TrackingEventFamily::LiveMode,
        true,
        true,
        true,
        true,
    ),
    spec(
        "tracking.live-mode.stopped",
        TrackingEventFamily::LiveMode,
        true,
        true,
        true,
        true,
    ),
    spec(
        "notification.intent.created",
        TrackingEventFamily::Notification,
        true,
        true,
        true,
        false,
    ),
    spec(
        "notification.dispatch.requested",
        TrackingEventFamily::Notification,
        true,
        true,
        true,
        false,
    ),
    spec(
        "notification.dispatch.result-observed",
        TrackingEventFamily::Notification,
        true,
        true,
        true,
        false,
    ),
    spec(
        "escalation.intent.created",
        TrackingEventFamily::Escalation,
        true,
        true,
        true,
        false,
    ),
    spec(
        "escalation.result-observed",
        TrackingEventFamily::Escalation,
        true,
        true,
        true,
        false,
    ),
];

const fn spec(
    event_type: &'static str,
    family: TrackingEventFamily,
    requires_causation: bool,
    requires_evidence_ref: bool,
    requires_policy_ref: bool,
    requires_live_mode_ttl: bool,
) -> TrackingEventContractSpec {
    TrackingEventContractSpec {
        event_type,
        family,
        requires_causation,
        requires_evidence_ref,
        requires_policy_ref,
        requires_live_mode_ttl,
    }
}

pub fn tracking_event_contract(event_type: &str) -> Option<&'static TrackingEventContractSpec> {
    TRACKING_EVENT_CONTRACTS
        .iter()
        .find(|specification| specification.event_type == event_type)
}

pub fn validate_tracking_event_contract(
    event_type: &str,
    input: TrackingEventContractInput<'_>,
) -> Result<(), TrackingEventContractError> {
    let specification =
        tracking_event_contract(event_type).ok_or(TrackingEventContractError::UnknownEventType)?;
    required(input.event_id, TrackingEventContractError::MissingEventId)?;
    required(
        input.correlation_id,
        TrackingEventContractError::MissingCorrelationId,
    )?;
    required(
        input.aggregate_key,
        TrackingEventContractError::MissingAggregateKey,
    )?;
    required(
        input.idempotency_key,
        TrackingEventContractError::MissingIdempotencyKey,
    )?;
    if specification.requires_causation {
        required(
            input.causation_id,
            TrackingEventContractError::MissingCausationId,
        )?;
    }
    if specification.requires_evidence_ref {
        required(
            input.evidence_ref,
            TrackingEventContractError::MissingEvidenceRef,
        )?;
    }
    if specification.requires_policy_ref {
        required(
            input.policy_ref,
            TrackingEventContractError::MissingPolicyRef,
        )?;
    }
    if specification.requires_live_mode_ttl {
        if input.live_mode_ttl_seconds.unwrap_or_default() == 0 {
            return Err(TrackingEventContractError::MissingLiveModeTtl);
        }
        required(input.audit_ref, TrackingEventContractError::MissingAuditRef)?;
    }
    if specification.family == TrackingEventFamily::Ai {
        required(
            input.uncertainty_state,
            TrackingEventContractError::MissingUncertaintyState,
        )?;
        if input.ai_authority_field_present {
            return Err(TrackingEventContractError::AiAuthorityFieldForbidden);
        }
    }
    Ok(())
}

fn required(
    value: Option<&str>,
    error: TrackingEventContractError,
) -> Result<(), TrackingEventContractError> {
    match value.filter(|value| !value.trim().is_empty()) {
        Some(_) => Ok(()),
        None => Err(error),
    }
}
