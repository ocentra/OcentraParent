use super::super::super::assessment_reasons::MergeDecisionReason;

pub(super) fn label(reason: MergeDecisionReason) -> Option<&'static str> {
    match reason {
        MergeDecisionReason::SameCanonicalDeviceId => Some("same-canonical-device-id"),
        MergeDecisionReason::SharedInstallId => Some("shared-install-id"),
        MergeDecisionReason::SharedPairingId => Some("shared-pairing-id"),
        MergeDecisionReason::SharedStableMac => Some("shared-stable-mac"),
        MergeDecisionReason::SharedMdnsInstanceName => Some("shared-mdns-instance-name"),
        MergeDecisionReason::SharedSsdpUdn => Some("shared-ssdp-udn"),
        _ => None,
    }
}
