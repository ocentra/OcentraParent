use super::{
    NetworkHardeningReadinessProof, NetworkReadinessGate, NetworkRetentionReadinessProof,
    NetworkRolloutReadinessProof, NetworkSupportReadinessProof,
};

pub(super) fn readiness_gates() -> Vec<NetworkReadinessGate> {
    vec![
        NetworkReadinessGate::SecurityThreatModel,
        NetworkReadinessGate::PrivacyAndCompliance,
        NetworkReadinessGate::RetentionDeleteExport,
        NetworkReadinessGate::KeyAndSecretHandling,
        NetworkReadinessGate::ProvenanceAndRollback,
        NetworkReadinessGate::DeploymentRollback,
        NetworkReadinessGate::SupportAndTraining,
        NetworkReadinessGate::StagedRollout,
        NetworkReadinessGate::KnownGapSignoff,
    ]
}

pub(super) fn retention_refs(proof: NetworkRetentionReadinessProof) -> Vec<String> {
    vec![
        proof.encryption_at_rest_ref,
        proof.quota_rotation_ref,
        proof.retention_policy_ref,
        proof.delete_export_ref,
        proof.custody_ref,
        proof.private_family_traffic_exclusion_ref,
    ]
}

pub(super) fn hardening_refs(proof: NetworkHardeningReadinessProof) -> Vec<String> {
    vec![
        proof.key_rotation_ref,
        proof.secret_handling_ref,
        proof.rule_set_provenance_ref,
        proof.rule_set_rollback_ref,
        proof.ai_model_version_promotion_ref,
        proof.ai_model_rollback_ref,
    ]
}

pub(super) fn support_refs(proof: NetworkSupportReadinessProof) -> Vec<String> {
    vec![
        proof.parent_guide_ref,
        proof.user_guide_ref,
        proof.faq_ref,
        proof.support_playbook_ref,
        proof.staff_training_ref,
    ]
}

pub(super) fn rollout_refs(proof: NetworkRolloutReadinessProof) -> Vec<String> {
    vec![
        proof.deployment_runbook_ref,
        proof.rollback_runbook_ref,
        proof.staged_rollout_plan_ref,
        proof.monitoring_ref,
        proof.incident_response_ref,
        proof.known_gap_signoff_ref,
    ]
}
