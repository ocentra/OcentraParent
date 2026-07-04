use ocentra_network_evidence::ai_audit::*;
use ocentra_network_evidence::risk_budget::*;

use super::{audit, AuditFixtureCase, PriorEventRefCase, SignalRefCase};

pub(super) fn risk_signal(
    signal_ref: SignalRefCase,
    audit_report: NetworkAiAuditReport,
    evidence_tier: NetworkRiskBudgetEvidenceTier,
    base_risk_points: u16,
    safe_behavior_credit_points: u16,
) -> NetworkRiskBudgetSignal {
    let signal_ref = match signal_ref {
        SignalRefCase::Block => "network-risk-signal-1",
        SignalRefCase::Manual => "network-risk-signal-manual",
        SignalRefCase::Safe => "network-risk-signal-safe",
        SignalRefCase::MissingProof => "network-risk-signal-missing-proof",
        SignalRefCase::Signature => "network-risk-signal-signature",
        SignalRefCase::Unsupported => "network-risk-signal-unsupported",
        SignalRefCase::Low => "network-risk-signal-low",
    };

    NetworkRiskBudgetSignal {
        signal_ref: signal_ref.to_owned(),
        audit_report,
        evidence_tier,
        base_risk_points,
        safe_behavior_credit_points,
        known_safe: false,
        expected_activity: false,
        signature_only: false,
    }
}

pub(super) fn low_risk_signal() -> NetworkRiskBudgetSignal {
    risk_signal(
        SignalRefCase::Low,
        audit::benign_audit_report(AuditFixtureCase::Low),
        NetworkRiskBudgetEvidenceTier::AiAuditWithCitations,
        10,
        0,
    )
}

pub(super) fn prior_event(
    event_ref: PriorEventRefCase,
    risk_points: u16,
    within_window: bool,
    same_household_rule: bool,
) -> NetworkRiskBudgetPriorEvent {
    let event_ref = match event_ref {
        PriorEventRefCase::Block => "prior-network-risk-1",
        PriorEventRefCase::Safe => "prior-network-risk-safe",
        PriorEventRefCase::OutsideWindow => "prior-network-risk-outside-window",
    };

    NetworkRiskBudgetPriorEvent {
        event_ref: event_ref.to_owned(),
        risk_points,
        within_window,
        same_household_rule,
    }
}
