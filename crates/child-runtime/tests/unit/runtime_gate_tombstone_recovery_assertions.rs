use ocentra_child_runtime::service::{
    storage_custody_runtime::ChildStorageCustodyOutcome, ChildAgentIngressError,
    ChildAgentServiceError,
};

pub(super) fn require_manual_required_custody_rejection(
    result: Result<ChildStorageCustodyOutcome, ChildAgentIngressError>,
) -> Result<(), Box<dyn std::error::Error>> {
    match result {
        Err(ChildAgentIngressError::Service(error)) => match *error {
            ChildAgentServiceError::TrustBindingManualRequired => Ok(()),
            unexpected => {
                Err(format!("unexpected custody ingress service error: {unexpected:?}").into())
            }
        },
        Err(unexpected) => Err(format!("unexpected custody ingress error: {unexpected:?}").into()),
        Ok(_) => Err("custody ingress accepted an action without owner binding".into()),
    }
}
