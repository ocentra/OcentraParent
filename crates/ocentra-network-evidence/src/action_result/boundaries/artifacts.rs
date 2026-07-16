use super::*;

pub(super) fn missing_required_artifacts(
    artifacts: &NetworkActionResultArtifactRefs,
) -> Vec<NetworkActionResultRequiredArtifact> {
    let mut missing = Vec::new();
    push_missing(
        &mut missing,
        artifacts.adapter_proof_ref.as_ref(),
        NetworkActionResultRequiredArtifact::AdapterProof,
    );
    push_missing(
        &mut missing,
        artifacts.apply_artifact_ref.as_ref(),
        NetworkActionResultRequiredArtifact::ApplyArtifact,
    );
    push_missing(
        &mut missing,
        artifacts.result_artifact_ref.as_ref(),
        NetworkActionResultRequiredArtifact::ResultArtifact,
    );
    push_missing(
        &mut missing,
        artifacts.audit_event_ref.as_ref(),
        NetworkActionResultRequiredArtifact::AuditEvent,
    );
    missing
}

fn push_missing(
    missing: &mut Vec<NetworkActionResultRequiredArtifact>,
    value: Option<&String>,
    artifact: NetworkActionResultRequiredArtifact,
) {
    if value.is_none() {
        missing.push(artifact);
    }
}
