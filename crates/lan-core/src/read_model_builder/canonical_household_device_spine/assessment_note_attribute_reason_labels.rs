use super::super::super::assessment_reasons::MergeDecisionReason;

pub(super) fn label(reason: MergeDecisionReason) -> &'static str {
    match reason {
        MergeDecisionReason::SharedLocalServiceIdentityAnchor => {
            "shared-local-service-identity-anchor"
        }
        MergeDecisionReason::SharedIpAddress => "shared-ip-address",
        MergeDecisionReason::SharedHostname => "shared-hostname",
        MergeDecisionReason::SharedVendor => "shared-vendor",
        MergeDecisionReason::SharedDeviceType => "shared-device-type",
        MergeDecisionReason::ConflictingOcentraDeviceId => "conflicting-ocentra-device-id",
        MergeDecisionReason::ConflictingChildProfileId => "conflicting-child-profile-id",
        _ => super::identity_reason_labels::label(reason).unwrap_or_default(),
    }
}
