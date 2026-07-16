use super::protocol_lookup;
use crate::{constants, BrowserInterventionDeliveryState};

impl BrowserInterventionDeliveryState {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (
                    constants::browser::INTERVENTION_DELIVERY_NOT_DELIVERED,
                    Self::NotDelivered,
                ),
                (
                    constants::browser::INTERVENTION_DELIVERY_WARN_PAGE_RENDERED,
                    Self::WarnPageRendered,
                ),
                (
                    constants::browser::INTERVENTION_DELIVERY_BLOCK_PAGE_RENDERED,
                    Self::BlockPageRendered,
                ),
                (
                    constants::browser::INTERVENTION_DELIVERY_APPROVAL_HOLD_RENDERED,
                    Self::ApprovalHoldRendered,
                ),
                (
                    constants::browser::INTERVENTION_DELIVERY_CHECKING_HOLD_RENDERED,
                    Self::CheckingHoldRendered,
                ),
                (
                    constants::browser::INTERVENTION_DELIVERY_PORTAL_ROW_ONLY,
                    Self::PortalRowOnly,
                ),
                (
                    constants::browser::INTERVENTION_DELIVERY_MANUAL_REQUIRED,
                    Self::ManualRequired,
                ),
            ],
        )
    }
}
