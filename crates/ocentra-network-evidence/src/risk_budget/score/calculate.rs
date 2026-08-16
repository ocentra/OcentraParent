use super::super::*;
use super::NetworkRiskBudgetScore;

pub(super) fn calculate_score(input: &NetworkRiskBudgetThresholdInput) -> NetworkRiskBudgetScore {
    let mut cited_signal_refs = Vec::new();
    let mut cited_audit_refs = Vec::new();
    let mut cited_evidence_refs = Vec::new();
    let mut cited_prior_event_refs = Vec::new();
    let mut active_signal_points = 0_u32;
    let mut requested_safe_credit_points = 0_u32;

    for signal in &input.signals {
        super::push_unique(&mut cited_signal_refs, &signal.signal_ref);
        super::push_unique(&mut cited_audit_refs, &signal.audit_report.audit_report_ref);
        for evidence_ref in &signal.audit_report.cited_evidence_refs {
            super::push_unique(&mut cited_evidence_refs, evidence_ref);
        }
        if signal.known_safe || signal.expected_activity {
            requested_safe_credit_points += u32::from(signal.safe_behavior_credit_points);
        } else {
            active_signal_points += u32::from(signal.base_risk_points);
        }
    }

    let mut prior_event_points = 0_u32;
    for prior_event in &input.prior_events {
        if prior_event.within_window && prior_event.same_household_rule {
            prior_event_points += u32::from(prior_event.risk_points);
            super::push_unique(&mut cited_prior_event_refs, &prior_event.event_ref);
        }
    }

    let age_profile_points =
        super::age_pressure_points(input.age_band) + u32::from(input.profile_risk_weight_points);
    let safe_behavior_credit_applied_points = requested_safe_credit_points.min(u32::from(
        input.household_policy.safe_behavior_credit_cap_points,
    ));
    let raw_points = age_profile_points + active_signal_points + prior_event_points;
    let total_risk_points = raw_points
        .saturating_sub(safe_behavior_credit_applied_points)
        .min(u32::from(input.thresholds.max_points));

    NetworkRiskBudgetScore {
        total_risk_points: total_risk_points as u16,
        age_profile_points: age_profile_points as u16,
        active_signal_points: active_signal_points as u16,
        prior_event_points: prior_event_points as u16,
        safe_behavior_credit_applied_points: safe_behavior_credit_applied_points as u16,
        cited_signal_refs,
        cited_audit_refs,
        cited_evidence_refs,
        cited_prior_event_refs,
    }
}
