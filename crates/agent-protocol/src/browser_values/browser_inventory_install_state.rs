use super::protocol_lookup;
use crate::{constants, BrowserInventoryInstallState};

impl BrowserInventoryInstallState {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (
                    constants::browser::INVENTORY_INSTALL_STATE_INSTALLED,
                    Self::Installed,
                ),
                (
                    constants::browser::INVENTORY_INSTALL_STATE_NOT_INSTALLED,
                    Self::NotInstalled,
                ),
                (
                    constants::browser::INVENTORY_INSTALL_STATE_CANDIDATE_RUNNING,
                    Self::CandidateRunning,
                ),
                (
                    constants::browser::INVENTORY_INSTALL_STATE_PACKAGED,
                    Self::Packaged,
                ),
                (
                    constants::browser::INVENTORY_INSTALL_STATE_PORTABLE,
                    Self::Portable,
                ),
                (
                    constants::browser::INVENTORY_INSTALL_STATE_UNKNOWN,
                    Self::Unknown,
                ),
            ],
        )
    }
}
