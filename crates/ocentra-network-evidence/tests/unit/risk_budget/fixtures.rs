use ocentra_network_evidence::ai_audit::*;
use ocentra_network_evidence::risk_budget::*;

mod audit;
mod detection;
mod signals;
mod threshold;

#[derive(Clone, Copy)]
pub(super) enum SignalRefCase {
    Block,
    Manual,
    Safe,
    MissingProof,
    Signature,
    Unsupported,
    Low,
}

#[derive(Clone, Copy)]
pub(super) enum PriorEventRefCase {
    Block,
    Safe,
    OutsideWindow,
}

#[derive(Clone, Copy)]
pub(super) enum AuditFixtureCase {
    Block,
    Manual,
    Safe,
    MissingProof,
    Signature,
    Unsupported,
    Low,
}

pub(super) fn threshold_input(
    signals: Vec<NetworkRiskBudgetSignal>,
    prior_events: Vec<NetworkRiskBudgetPriorEvent>,
    household_policy: NetworkRiskBudgetHouseholdPolicy,
    adapter_proof_state: NetworkRiskBudgetAdapterProofState,
) -> NetworkRiskBudgetThresholdInput {
    threshold::threshold_input(signals, prior_events, household_policy, adapter_proof_state)
}

pub(super) fn default_policy() -> NetworkRiskBudgetHouseholdPolicy {
    threshold::default_policy()
}

pub(super) fn risk_signal(
    signal_ref: SignalRefCase,
    audit_report: NetworkAiAuditReport,
    evidence_tier: NetworkRiskBudgetEvidenceTier,
    base_risk_points: u16,
    safe_behavior_credit_points: u16,
) -> NetworkRiskBudgetSignal {
    signals::risk_signal(
        signal_ref,
        audit_report,
        evidence_tier,
        base_risk_points,
        safe_behavior_credit_points,
    )
}

pub(super) fn low_risk_signal() -> NetworkRiskBudgetSignal {
    signals::low_risk_signal()
}

pub(super) fn prior_event(
    event_ref: PriorEventRefCase,
    risk_points: u16,
    within_window: bool,
    same_household_rule: bool,
) -> NetworkRiskBudgetPriorEvent {
    signals::prior_event(event_ref, risk_points, within_window, same_household_rule)
}

pub(super) fn high_risk_audit_report(fixture: AuditFixtureCase) -> NetworkAiAuditReport {
    audit::high_risk_audit_report(fixture)
}

pub(super) fn benign_audit_report(fixture: AuditFixtureCase) -> NetworkAiAuditReport {
    audit::benign_audit_report(fixture)
}
