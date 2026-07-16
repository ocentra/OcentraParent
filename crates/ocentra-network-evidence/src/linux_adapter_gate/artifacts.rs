use super::normalization::NetworkLinuxAdapterArtifactRefs;
use super::{NetworkLinuxAdapterGateError, NetworkLinuxAdapterRequiredArtifact};

pub(super) fn normalize_artifact_refs(
    input: &super::NetworkLinuxAdapterGateInput,
) -> Result<NetworkLinuxAdapterArtifactRefs, NetworkLinuxAdapterGateError> {
    Ok(NetworkLinuxAdapterArtifactRefs {
        distro_kernel_proof_ref: normalized_artifact_ref(
            input.distro_kernel_proof_ref.as_deref(),
            NetworkLinuxAdapterRequiredArtifact::DistroKernelProof,
        )?,
        permission_proof_ref: normalized_artifact_ref(
            input.permission_proof_ref.as_deref(),
            NetworkLinuxAdapterRequiredArtifact::PermissionProof,
        )?,
        adapter_api_capability_proof_ref: normalized_artifact_ref(
            input.adapter_api_capability_proof_ref.as_deref(),
            NetworkLinuxAdapterRequiredArtifact::AdapterApiCapabilityProof,
        )?,
        adapter_plan_proof_ref: normalized_artifact_ref(
            input.adapter_plan_proof_ref.as_deref(),
            NetworkLinuxAdapterRequiredArtifact::AdapterPlanProof,
        )?,
        service_manager_scope_proof_ref: normalized_artifact_ref(
            input.service_manager_scope_proof_ref.as_deref(),
            NetworkLinuxAdapterRequiredArtifact::ServiceManagerScopeProof,
        )?,
        rollback_plan_ref: normalized_artifact_ref(
            input.rollback_plan_ref.as_deref(),
            NetworkLinuxAdapterRequiredArtifact::RollbackPlan,
        )?,
        lab_result_artifact_ref: normalized_artifact_ref(
            input.lab_result_artifact_ref.as_deref(),
            NetworkLinuxAdapterRequiredArtifact::LabResultArtifact,
        )?,
        audit_event_ref: normalized_artifact_ref(
            input.audit_event_ref.as_deref(),
            NetworkLinuxAdapterRequiredArtifact::AuditEvent,
        )?,
    })
}

pub(super) fn missing_required_artifacts(
    artifacts: &NetworkLinuxAdapterArtifactRefs,
) -> Vec<NetworkLinuxAdapterRequiredArtifact> {
    let mut missing = Vec::new();
    push_missing(
        &mut missing,
        artifacts.distro_kernel_proof_ref.as_ref(),
        NetworkLinuxAdapterRequiredArtifact::DistroKernelProof,
    );
    push_missing(
        &mut missing,
        artifacts.permission_proof_ref.as_ref(),
        NetworkLinuxAdapterRequiredArtifact::PermissionProof,
    );
    push_missing(
        &mut missing,
        artifacts.adapter_api_capability_proof_ref.as_ref(),
        NetworkLinuxAdapterRequiredArtifact::AdapterApiCapabilityProof,
    );
    push_missing(
        &mut missing,
        artifacts.adapter_plan_proof_ref.as_ref(),
        NetworkLinuxAdapterRequiredArtifact::AdapterPlanProof,
    );
    push_missing(
        &mut missing,
        artifacts.service_manager_scope_proof_ref.as_ref(),
        NetworkLinuxAdapterRequiredArtifact::ServiceManagerScopeProof,
    );
    push_missing(
        &mut missing,
        artifacts.rollback_plan_ref.as_ref(),
        NetworkLinuxAdapterRequiredArtifact::RollbackPlan,
    );
    push_missing(
        &mut missing,
        artifacts.lab_result_artifact_ref.as_ref(),
        NetworkLinuxAdapterRequiredArtifact::LabResultArtifact,
    );
    push_missing(
        &mut missing,
        artifacts.audit_event_ref.as_ref(),
        NetworkLinuxAdapterRequiredArtifact::AuditEvent,
    );
    missing
}

fn push_missing(
    missing: &mut Vec<NetworkLinuxAdapterRequiredArtifact>,
    value: Option<&String>,
    artifact: NetworkLinuxAdapterRequiredArtifact,
) {
    if value.is_none() {
        missing.push(artifact);
    }
}

fn normalized_artifact_ref(
    value: Option<&str>,
    artifact: NetworkLinuxAdapterRequiredArtifact,
) -> Result<Option<String>, NetworkLinuxAdapterGateError> {
    match value {
        Some(raw) => normalize_ref(raw).map(Some).ok_or(
            NetworkLinuxAdapterGateError::EmptyRequiredArtifactRef(artifact),
        ),
        None => Ok(None),
    }
}

fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}
