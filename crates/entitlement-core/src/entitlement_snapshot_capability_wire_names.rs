#![forbid(unsafe_code)]

use crate::entitlement_access::EntitlementCapability;

pub(super) fn capability_wire_name(capability: EntitlementCapability) -> &'static str {
    match capability {
        EntitlementCapability::Tracking => super::CAPABILITY_TRACKING,
        EntitlementCapability::ScreenEvidence => super::CAPABILITY_SCREEN_EVIDENCE,
        EntitlementCapability::RemoteAccess => super::CAPABILITY_REMOTE_ACCESS,
        EntitlementCapability::Enforcement => super::CAPABILITY_ENFORCEMENT,
    }
}
