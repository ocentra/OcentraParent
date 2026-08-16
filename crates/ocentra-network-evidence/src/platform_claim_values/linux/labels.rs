use crate::NetworkLinuxAdapterRequiredArtifact;

pub(super) fn linux_artifact_label(artifact: NetworkLinuxAdapterRequiredArtifact) -> &'static str {
    match artifact {
        NetworkLinuxAdapterRequiredArtifact::DistroKernelProof => "linux-adapter.distro-kernel",
        NetworkLinuxAdapterRequiredArtifact::PermissionProof => "linux-adapter.permission",
        NetworkLinuxAdapterRequiredArtifact::AdapterApiCapabilityProof => {
            "linux-adapter.api-capability"
        }
        NetworkLinuxAdapterRequiredArtifact::AdapterPlanProof => "linux-adapter.plan",
        NetworkLinuxAdapterRequiredArtifact::ServiceManagerScopeProof => {
            "linux-adapter.service-manager"
        }
        NetworkLinuxAdapterRequiredArtifact::RollbackPlan => "linux-adapter.rollback-plan",
        NetworkLinuxAdapterRequiredArtifact::LabResultArtifact => "linux-adapter.lab-result",
        NetworkLinuxAdapterRequiredArtifact::AuditEvent => "linux-adapter.audit-event",
    }
}
