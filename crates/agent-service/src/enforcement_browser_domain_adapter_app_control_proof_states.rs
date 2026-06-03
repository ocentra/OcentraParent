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

struct AppControlStateInput {
    proof_state_id: &'static str,
    readiness_state: V08WindowsAppControlReadinessState,
    policy_mutation_state: V08WindowsAppControlPolicyMutationState,
    rule_identity_kinds: Vec<V08WindowsAppControlRuleIdentityKind>,
    admin_requirement: V08WindowsAppControlAdminRequirement,
    event_states: Vec<V08WindowsAppControlEventState>,
    manual_proof_requirements: Vec<&'static str>,
    text: AppControlBoundaryText,
    generated_at: String,
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
    app_control_state(AppControlStateInput {
        proof_state_id: proof::STATE_ID_APP_CONTROL_READINESS,
        readiness_state: V08WindowsAppControlReadinessState::ReadinessCheck,
        policy_mutation_state: V08WindowsAppControlPolicyMutationState::DetectOnly,
        rule_identity_kinds: identity_kinds(),
        admin_requirement: V08WindowsAppControlAdminRequirement::AdministratorRequired,
        event_states: vec![V08WindowsAppControlEventState::ManualProofRequired],
        manual_proof_requirements: vec![
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_EDITION,
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_ADMIN,
        ],
        text: app_control_text(
            proof::CLAIM_WINDOWS_APP_CONTROL_READINESS,
            proof::FALLBACK_WINDOWS_APP_CONTROL_READINESS,
        ),
        generated_at: generated_at.to_string(),
    })
}

fn audit_only_state(generated_at: &str) -> V08WindowsAppControlProofState {
    app_control_state(AppControlStateInput {
        proof_state_id: proof::STATE_ID_APP_CONTROL_AUDIT_ONLY,
        readiness_state: V08WindowsAppControlReadinessState::AuditOnly,
        policy_mutation_state: V08WindowsAppControlPolicyMutationState::AuditOnlyVisible,
        rule_identity_kinds: identity_kinds(),
        admin_requirement: V08WindowsAppControlAdminRequirement::AdministratorRequired,
        event_states: vec![V08WindowsAppControlEventState::AuditVisible],
        manual_proof_requirements: vec![
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_AUDIT_POLICY,
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_AUDIT_QUERY,
        ],
        text: app_control_text(
            proof::CLAIM_WINDOWS_APP_CONTROL_AUDIT_ONLY,
            proof::FALLBACK_WINDOWS_APP_CONTROL_AUDIT_ONLY,
        ),
        generated_at: generated_at.to_string(),
    })
}

fn enforced_state(generated_at: &str) -> V08WindowsAppControlProofState {
    app_control_state(AppControlStateInput {
        proof_state_id: proof::STATE_ID_APP_CONTROL_ENFORCED,
        readiness_state: V08WindowsAppControlReadinessState::Enforced,
        policy_mutation_state: V08WindowsAppControlPolicyMutationState::CreateUpdateManualRequired,
        rule_identity_kinds: identity_kinds(),
        admin_requirement: V08WindowsAppControlAdminRequirement::AdministratorRequired,
        event_states: vec![
            V08WindowsAppControlEventState::AuditVisible,
            V08WindowsAppControlEventState::RollbackVisible,
        ],
        manual_proof_requirements: vec![
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_ENFORCED_POLICY,
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_POLICY_REFRESH,
            proof::REQUIREMENT_ROLLBACK,
        ],
        text: app_control_text(
            proof::CLAIM_WINDOWS_APP_CONTROL_ENFORCED,
            proof::FALLBACK_WINDOWS_APP_CONTROL_ENFORCED,
        ),
        generated_at: generated_at.to_string(),
    })
}

fn manual_required_state(generated_at: &str) -> V08WindowsAppControlProofState {
    app_control_state(AppControlStateInput {
        proof_state_id: proof::STATE_ID_APP_CONTROL_MANUAL_REQUIRED,
        readiness_state: V08WindowsAppControlReadinessState::ManualRequired,
        policy_mutation_state: V08WindowsAppControlPolicyMutationState::ManualSetupRequired,
        rule_identity_kinds: identity_kinds(),
        admin_requirement: V08WindowsAppControlAdminRequirement::ManualOperatorRequired,
        event_states: vec![V08WindowsAppControlEventState::ManualProofRequired],
        manual_proof_requirements: vec![
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_MANUAL_SETUP,
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_OPERATOR_CONFIRMATION,
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_IDENTITY_REVIEW,
        ],
        text: app_control_text(
            proof::CLAIM_WINDOWS_APP_CONTROL_MANUAL_REQUIRED,
            proof::FALLBACK_WINDOWS_APP_CONTROL_MANUAL_REQUIRED,
        ),
        generated_at: generated_at.to_string(),
    })
}

fn unavailable_state(generated_at: &str) -> V08WindowsAppControlProofState {
    app_control_state(AppControlStateInput {
        proof_state_id: proof::STATE_ID_APP_CONTROL_UNAVAILABLE,
        readiness_state: V08WindowsAppControlReadinessState::Unavailable,
        policy_mutation_state: V08WindowsAppControlPolicyMutationState::Unavailable,
        rule_identity_kinds: Vec::new(),
        admin_requirement: V08WindowsAppControlAdminRequirement::ServicePermissionRequired,
        event_states: vec![V08WindowsAppControlEventState::Unavailable],
        manual_proof_requirements: vec![
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_PROVIDER,
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_PERMISSION_DENIAL,
        ],
        text: app_control_text(
            proof::CLAIM_WINDOWS_APP_CONTROL_UNAVAILABLE,
            proof::FALLBACK_WINDOWS_APP_CONTROL_UNAVAILABLE,
        ),
        generated_at: generated_at.to_string(),
    })
}

fn failed_state(generated_at: &str) -> V08WindowsAppControlProofState {
    app_control_state(AppControlStateInput {
        proof_state_id: proof::STATE_ID_APP_CONTROL_FAILED,
        readiness_state: V08WindowsAppControlReadinessState::Failed,
        policy_mutation_state: V08WindowsAppControlPolicyMutationState::Failed,
        rule_identity_kinds: identity_kinds(),
        admin_requirement: V08WindowsAppControlAdminRequirement::AdministratorRequired,
        event_states: vec![V08WindowsAppControlEventState::FailureVisible],
        manual_proof_requirements: vec![
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_FAILURE_EVENT,
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_IDENTITY_FAILURE,
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_AUDIT_FAILURE,
        ],
        text: app_control_text(
            proof::CLAIM_WINDOWS_APP_CONTROL_FAILED,
            proof::FALLBACK_WINDOWS_APP_CONTROL_FAILED,
        ),
        generated_at: generated_at.to_string(),
    })
}

fn app_control_state(input: AppControlStateInput) -> V08WindowsAppControlProofState {
    V08WindowsAppControlProofState {
        proof_state_id: input.proof_state_id.to_string(),
        readiness_state: input.readiness_state,
        policy_mutation_state: input.policy_mutation_state,
        rule_identity_kinds: input.rule_identity_kinds,
        admin_requirement: input.admin_requirement,
        event_states: input.event_states,
        manual_proof_requirements: input
            .manual_proof_requirements
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        claim_boundary: input.text.claim_boundary.to_string(),
        fallback_behavior: input.text.fallback_behavior.to_string(),
        app_control_prevention_claimed: false,
        policy_creation_claimed: false,
        policy_update_claimed: false,
        rollback_claimed: false,
        last_checked_at: input.generated_at,
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
