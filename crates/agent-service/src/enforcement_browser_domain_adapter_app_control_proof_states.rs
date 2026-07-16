use ocentra_parent_agent_protocol::constants::v08_browser_domain_adapter_proof as proof;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08WindowsAppControlAdminRequirement;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08WindowsAppControlEventState;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08WindowsAppControlPolicyMutationState;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08WindowsAppControlProofState;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08WindowsAppControlReadinessState;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08WindowsAppControlRuleIdentityKind;

struct AppControlBoundaryText {
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
}

#[derive(Clone, Copy)]
pub(crate) struct GeneratedAtTextRef<'a>(pub(crate) &'a str);

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

pub(crate) fn app_control_state_specs(
    generated_at: GeneratedAtTextRef<'_>,
) -> Vec<V08WindowsAppControlProofState> {
    vec![
        readiness_state(generated_at),
        audit_only_state(generated_at),
        enforced_state(generated_at),
        manual_required_state(generated_at),
        unavailable_state(generated_at),
        failed_state(generated_at),
    ]
}

fn readiness_state(generated_at: GeneratedAtTextRef<'_>) -> V08WindowsAppControlProofState {
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
        text: AppControlBoundaryText {
            claim_boundary: proof::CLAIM_WINDOWS_APP_CONTROL_READINESS,
            fallback_behavior: proof::FALLBACK_WINDOWS_APP_CONTROL_READINESS,
        },
        generated_at: generated_at.0.to_string(),
    })
}

fn audit_only_state(generated_at: GeneratedAtTextRef<'_>) -> V08WindowsAppControlProofState {
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
        text: AppControlBoundaryText {
            claim_boundary: proof::CLAIM_WINDOWS_APP_CONTROL_AUDIT_ONLY,
            fallback_behavior: proof::FALLBACK_WINDOWS_APP_CONTROL_AUDIT_ONLY,
        },
        generated_at: generated_at.0.to_string(),
    })
}

fn enforced_state(generated_at: GeneratedAtTextRef<'_>) -> V08WindowsAppControlProofState {
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
        text: AppControlBoundaryText {
            claim_boundary: proof::CLAIM_WINDOWS_APP_CONTROL_ENFORCED,
            fallback_behavior: proof::FALLBACK_WINDOWS_APP_CONTROL_ENFORCED,
        },
        generated_at: generated_at.0.to_string(),
    })
}

fn manual_required_state(generated_at: GeneratedAtTextRef<'_>) -> V08WindowsAppControlProofState {
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
        text: AppControlBoundaryText {
            claim_boundary: proof::CLAIM_WINDOWS_APP_CONTROL_MANUAL_REQUIRED,
            fallback_behavior: proof::FALLBACK_WINDOWS_APP_CONTROL_MANUAL_REQUIRED,
        },
        generated_at: generated_at.0.to_string(),
    })
}

fn unavailable_state(generated_at: GeneratedAtTextRef<'_>) -> V08WindowsAppControlProofState {
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
        text: AppControlBoundaryText {
            claim_boundary: proof::CLAIM_WINDOWS_APP_CONTROL_UNAVAILABLE,
            fallback_behavior: proof::FALLBACK_WINDOWS_APP_CONTROL_UNAVAILABLE,
        },
        generated_at: generated_at.0.to_string(),
    })
}

fn failed_state(generated_at: GeneratedAtTextRef<'_>) -> V08WindowsAppControlProofState {
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
        text: AppControlBoundaryText {
            claim_boundary: proof::CLAIM_WINDOWS_APP_CONTROL_FAILED,
            fallback_behavior: proof::FALLBACK_WINDOWS_APP_CONTROL_FAILED,
        },
        generated_at: generated_at.0.to_string(),
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
