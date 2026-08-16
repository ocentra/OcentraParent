use super::*;

pub(super) fn normalize_artifact_refs(
    input: &NetworkActionResultInput,
) -> Result<NetworkActionResultArtifactRefs, NetworkActionResultError> {
    Ok(NetworkActionResultArtifactRefs {
        adapter_proof_ref: refs::normalized_artifact_ref(
            input.adapter_proof_ref.as_deref(),
            NetworkActionResultRequiredArtifact::AdapterProof,
        )?,
        apply_artifact_ref: refs::normalized_artifact_ref(
            input.apply_artifact_ref.as_deref(),
            NetworkActionResultRequiredArtifact::ApplyArtifact,
        )?,
        result_artifact_ref: refs::normalized_artifact_ref(
            input.result_artifact_ref.as_deref(),
            NetworkActionResultRequiredArtifact::ResultArtifact,
        )?,
        audit_event_ref: refs::normalized_artifact_ref(
            input.audit_event_ref.as_deref(),
            NetworkActionResultRequiredArtifact::AuditEvent,
        )?,
    })
}
