use ocentra_parent_agent_protocol::{
    constants::v08_browser_domain_adapter_proof as proof, V08WindowsAppControlAdminRequirement,
    V08WindowsAppControlEventState, V08WindowsAppControlPolicyMutationState,
    V08WindowsAppControlProofState, V08WindowsAppControlReadinessState,
    V08WindowsAppControlRuleIdentityKind,
};

struct AppControlBoundaryText {
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
}

pub(crate) fn app_control_state_specs(generated_at: &str) -> Vec<V08WindowsAppControlProofState> {
    vec![
        readiness_state(generated_at),
        audit_only_state(generated_at),
        enforced_state(generated_at),
        manual_required_state(generated_at),
        unavailable_state(generated_at),
        failed_state(generated_at),
    ]
}

fn readiness_state(generated_at: &str) -> V08WindowsAppControlProofState {
    app_control_state(
        proof::STATE_ID_APP_CONTROL_READINESS,
        V08WindowsAppControlReadinessState::ReadinessCheck,
        V08WindowsAppControlPolicyMutationState::DetectOnly,
        identity_kinds(),
        V08WindowsAppControlAdminRequirement::AdministratorRequired,
        vec![V08WindowsAppControlEventState::ManualProofRequired],
        vec![
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_EDITION,
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_ADMIN,
        ],
        app_control_text(
            proof::CLAIM_WINDOWS_APP_CONTROL_READINESS,
            proof::FALLBACK_WINDOWS_APP_CONTROL_READINESS,
        ),
        generated_at,
    )
}

fn audit_only_state(generated_at: &str) -> V08WindowsAppControlProofState {
    app_control_state(
        proof::STATE_ID_APP_CONTROL_AUDIT_ONLY,
        V08WindowsAppControlReadinessState::AuditOnly,
        V08WindowsAppControlPolicyMutationState::AuditOnlyVisible,
        identity_kinds(),
        V08WindowsAppControlAdminRequirement::AdministratorRequired,
        vec![V08WindowsAppControlEventState::AuditVisible],
        vec![
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_AUDIT_POLICY,
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_AUDIT_QUERY,
        ],
        app_control_text(
            proof::CLAIM_WINDOWS_APP_CONTROL_AUDIT_ONLY,
            proof::FALLBACK_WINDOWS_APP_CONTROL_AUDIT_ONLY,
        ),
        generated_at,
    )
}

fn enforced_state(generated_at: &str) -> V08WindowsAppControlProofState {
    app_control_state(
        proof::STATE_ID_APP_CONTROL_ENFORCED,
        V08WindowsAppControlReadinessState::Enforced,
        V08WindowsAppControlPolicyMutationState::CreateUpdateManualRequired,
        identity_kinds(),
        V08WindowsAppControlAdminRequirement::AdministratorRequired,
        vec![
            V08WindowsAppControlEventState::AuditVisible,
            V08WindowsAppControlEventState::RollbackVisible,
        ],
        vec![
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_ENFORCED_POLICY,
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_POLICY_REFRESH,
            proof::REQUIREMENT_ROLLBACK,
        ],
        app_control_text(
            proof::CLAIM_WINDOWS_APP_CONTROL_ENFORCED,
            proof::FALLBACK_WINDOWS_APP_CONTROL_ENFORCED,
        ),
        generated_at,
    )
}

fn manual_required_state(generated_at: &str) -> V08WindowsAppControlProofState {
    app_control_state(
        proof::STATE_ID_APP_CONTROL_MANUAL_REQUIRED,
        V08WindowsAppControlReadinessState::ManualRequired,
        V08WindowsAppControlPolicyMutationState::ManualSetupRequired,
        identity_kinds(),
        V08WindowsAppControlAdminRequirement::ManualOperatorRequired,
        vec![V08WindowsAppControlEventState::ManualProofRequired],
        vec![
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_MANUAL_SETUP,
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_OPERATOR_CONFIRMATION,
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_IDENTITY_REVIEW,
        ],
        app_control_text(
            proof::CLAIM_WINDOWS_APP_CONTROL_MANUAL_REQUIRED,
            proof::FALLBACK_WINDOWS_APP_CONTROL_MANUAL_REQUIRED,
        ),
        generated_at,
    )
}

fn unavailable_state(generated_at: &str) -> V08WindowsAppControlProofState {
    app_control_state(
        proof::STATE_ID_APP_CONTROL_UNAVAILABLE,
        V08WindowsAppControlReadinessState::Unavailable,
        V08WindowsAppControlPolicyMutationState::Unavailable,
        Vec::new(),
        V08WindowsAppControlAdminRequirement::ServicePermissionRequired,
        vec![V08WindowsAppControlEventState::Unavailable],
        vec![
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_PROVIDER,
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_PERMISSION_DENIAL,
        ],
        app_control_text(
            proof::CLAIM_WINDOWS_APP_CONTROL_UNAVAILABLE,
            proof::FALLBACK_WINDOWS_APP_CONTROL_UNAVAILABLE,
        ),
        generated_at,
    )
}

fn failed_state(generated_at: &str) -> V08WindowsAppControlProofState {
    app_control_state(
        proof::STATE_ID_APP_CONTROL_FAILED,
        V08WindowsAppControlReadinessState::Failed,
        V08WindowsAppControlPolicyMutationState::Failed,
        identity_kinds(),
        V08WindowsAppControlAdminRequirement::AdministratorRequired,
        vec![V08WindowsAppControlEventState::FailureVisible],
        vec![
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_FAILURE_EVENT,
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_IDENTITY_FAILURE,
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_AUDIT_FAILURE,
        ],
        app_control_text(
            proof::CLAIM_WINDOWS_APP_CONTROL_FAILED,
            proof::FALLBACK_WINDOWS_APP_CONTROL_FAILED,
        ),
        generated_at,
    )
}

fn app_control_state(
    proof_state_id: &'static str,
    readiness_state: V08WindowsAppControlReadinessState,
    policy_mutation_state: V08WindowsAppControlPolicyMutationState,
    rule_identity_kinds: Vec<V08WindowsAppControlRuleIdentityKind>,
    admin_requirement: V08WindowsAppControlAdminRequirement,
    event_states: Vec<V08WindowsAppControlEventState>,
    manual_proof_requirements: Vec<&'static str>,
    text: AppControlBoundaryText,
    generated_at: &str,
) -> V08WindowsAppControlProofState {
    V08WindowsAppControlProofState {
        proof_state_id: proof_state_id.to_string(),
        readiness_state,
        policy_mutation_state,
        rule_identity_kinds,
        admin_requirement,
        event_states,
        manual_proof_requirements: manual_proof_requirements
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        claim_boundary: text.claim_boundary.to_string(),
        fallback_behavior: text.fallback_behavior.to_string(),
        app_control_prevention_claimed: false,
        policy_creation_claimed: false,
        policy_update_claimed: false,
        rollback_claimed: false,
        last_checked_at: generated_at.to_string(),
    }
}

fn identity_kinds() -> Vec<V08WindowsAppControlRuleIdentityKind> {
    vec![
        V08WindowsAppControlRuleIdentityKind::Publisher,
        V08WindowsAppControlRuleIdentityKind::Path,
        V08WindowsAppControlRuleIdentityKind::Hash,
        V08WindowsAppControlRuleIdentityKind::Package,
    ]
}

fn app_control_text(
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
) -> AppControlBoundaryText {
    AppControlBoundaryText {
        claim_boundary,
        fallback_behavior,
    }
}
