use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus;
use ocentra_parent_agent_protocol::transport::PolicyRequestParentResolutionRequest;

use super::types::{
    AuditEventId, RejectionReason, RequestIdText, ResolutionSnapshot, SnapshotError,
};
use super::{domain, store};

const CONFIRMED_REQUEST_NOT_FOUND: &str = "confirmed-request-not-found";
const CONFIRMED_REQUEST_MALFORMED: &str = "confirmed-request-malformed";

pub(crate) async fn load(
    request: &PolicyRequestParentResolutionRequest,
) -> Result<ResolutionSnapshot, SnapshotError> {
    let confirmed_fields =
        store::load_audit_fields(AuditEventId(request.confirmed_audit_reference_id.clone()))
            .await
            .map_err(|error| {
                SnapshotError::new(
                    error.into_reason(),
                    false,
                    None,
                    PolicyRequestStatus::PreviewOnly,
                )
            })?
            .ok_or_else(|| {
                SnapshotError::new(
                    RejectionReason(CONFIRMED_REQUEST_NOT_FOUND.to_string()),
                    false,
                    None,
                    PolicyRequestStatus::PreviewOnly,
                )
            })?;
    let confirmed_request =
        domain::canonical_confirmed_request(&confirmed_fields).map_err(|error| {
            SnapshotError::new(
                RejectionReason(format!("{CONFIRMED_REQUEST_MALFORMED}: {error}")),
                true,
                None,
                PolicyRequestStatus::PreviewOnly,
            )
        })?;
    let previous_resolution = load_previous(request, &confirmed_request).await?;
    Ok(ResolutionSnapshot {
        confirmed_request,
        previous_resolution,
    })
}

async fn load_previous(
    request: &PolicyRequestParentResolutionRequest,
    confirmed_request: &ocentra_policy_control_core::policy_request::ChildPolicyRequest,
) -> Result<Option<super::types::PreviousResolution>, SnapshotError> {
    let fields =
        store::load_audit_fields(AuditEventId(request.approval_audit_reference_id.clone()))
            .await
            .map_err(|error| {
                SnapshotError::new(
                    error.into_reason(),
                    true,
                    Some(RequestIdText(
                        confirmed_request.request_id.as_str().to_string(),
                    )),
                    confirmed_request.status,
                )
            })?;
    let Some(fields) = fields else {
        return Ok(None);
    };
    domain::canonical_previous_resolution(&fields)
        .map(Some)
        .map_err(|error| {
            SnapshotError::new(
                RejectionReason(format!("{CONFIRMED_REQUEST_MALFORMED}: {error}")),
                true,
                Some(RequestIdText(
                    confirmed_request.request_id.as_str().to_string(),
                )),
                confirmed_request.status,
            )
        })
}
