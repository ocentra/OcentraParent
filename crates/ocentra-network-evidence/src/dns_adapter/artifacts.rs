use super::{NetworkDnsAdapterArtifactRefs, NetworkDnsAdapterRequiredArtifact};

pub(super) fn missing_required_artifacts(
    artifacts: &NetworkDnsAdapterArtifactRefs,
) -> Vec<NetworkDnsAdapterRequiredArtifact> {
    let mut missing = Vec::new();
    push_missing(
        &mut missing,
        artifacts.adapter_authorization_ref.as_ref(),
        NetworkDnsAdapterRequiredArtifact::AdapterAuthorization,
    );
    push_missing(
        &mut missing,
        artifacts.adapter_capability_proof_ref.as_ref(),
        NetworkDnsAdapterRequiredArtifact::CapabilityProof,
    );
    push_missing(
        &mut missing,
        artifacts.apply_artifact_ref.as_ref(),
        NetworkDnsAdapterRequiredArtifact::ApplyArtifact,
    );
    push_missing(
        &mut missing,
        artifacts.result_artifact_ref.as_ref(),
        NetworkDnsAdapterRequiredArtifact::ResultArtifact,
    );
    push_missing(
        &mut missing,
        artifacts.rollback_artifact_ref.as_ref(),
        NetworkDnsAdapterRequiredArtifact::RollbackArtifact,
    );
    push_missing(
        &mut missing,
        artifacts.audit_event_ref.as_ref(),
        NetworkDnsAdapterRequiredArtifact::AuditEvent,
    );
    missing
}

fn push_missing(
    missing: &mut Vec<NetworkDnsAdapterRequiredArtifact>,
    value: Option<&String>,
    artifact: NetworkDnsAdapterRequiredArtifact,
) {
    if value.is_none() {
        missing.push(artifact);
    }
}
