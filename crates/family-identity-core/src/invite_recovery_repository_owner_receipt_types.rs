use std::fmt;

use super::*;

macro_rules! redacted_owner_receipt_debug {
    ($type:ident) => {
        impl fmt::Debug for $type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify!($type))
                    .field("handoff_id", &self.handoff_id)
                    .field("correlation_id", &self.correlation_id)
                    .field("recovery_id", &self.recovery_id)
                    .field("attempt_id", &"<redacted>")
                    .field("transition_id", &"<redacted>")
                    .field("receipt_digest", &"<redacted>")
                    .finish()
            }
        }
    };
}

redacted_owner_receipt_debug!(ProviderCredentialSessionOwnerReceipt);
redacted_owner_receipt_debug!(DeviceTrustRevokeOwnerReceipt);
redacted_owner_receipt_debug!(DeviceTrustReinstallOwnerReceipt);
redacted_owner_receipt_debug!(HouseholdAuthorityMutationOwnerReceipt);
