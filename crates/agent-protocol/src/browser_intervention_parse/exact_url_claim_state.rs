use super::protocol_lookup;
use crate::{constants, BrowserExactUrlClaimState};

impl BrowserExactUrlClaimState {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (
                    constants::browser::INTERVENTION_EXACT_URL_PROVEN,
                    Self::ExactUrlProven,
                ),
                (
                    constants::browser::INTERVENTION_EXACT_URL_NOT_CLAIMED,
                    Self::NotClaimed,
                ),
                (
                    constants::browser::INTERVENTION_EXACT_URL_UNAVAILABLE,
                    Self::Unavailable,
                ),
            ],
        )
    }
}
