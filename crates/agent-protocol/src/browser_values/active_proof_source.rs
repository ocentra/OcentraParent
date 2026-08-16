use super::protocol_lookup;
use crate::{constants, BrowserActiveProofSource};

impl BrowserActiveProofSource {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (
                    constants::browser::ACTIVE_PROOF_SOURCE_TARGET_LIST_ONLY,
                    Self::TargetListOnly,
                ),
                (
                    constants::browser::ACTIVE_PROOF_SOURCE_CDP_FOCUS_ACTIVATION,
                    Self::CdpFocusActivation,
                ),
                (
                    constants::browser::ACTIVE_PROOF_SOURCE_MANAGED_EXTENSION_EVENT,
                    Self::ManagedExtensionEvent,
                ),
                (
                    constants::browser::ACTIVE_PROOF_SOURCE_FOREGROUND_CORRELATION,
                    Self::ForegroundCorrelation,
                ),
                (
                    constants::browser::ACTIVE_PROOF_SOURCE_OWNED_SHELL_EVENT,
                    Self::OwnedShellEvent,
                ),
            ],
        )
    }
}
