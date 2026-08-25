use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingRejectionReason, LanParentIntentEnvelope,
};

use super::TrustedDeviceRegistry;

mod entry;
mod selection;
mod shape;

impl TrustedDeviceRegistry {
    pub fn validate_intent_without_acceptance(
        &self,
        intent: &LanParentIntentEnvelope,
        origin: Option<&str>,
        observed_at: &str,
        require_selected_pairing: bool,
    ) -> Result<(), LanPairingRejectionReason> {
        self.validate_intent_without_consuming(
            intent,
            origin,
            observed_at,
            require_selected_pairing,
        )
    }

    pub(super) fn validate_intent_with_selection_requirement(
        &mut self,
        intent: &LanParentIntentEnvelope,
        origin: Option<&str>,
        observed_at: &str,
        require_selected_pairing: bool,
    ) -> Result<(), LanPairingRejectionReason> {
        self.validate_intent_without_consuming(
            intent,
            origin,
            observed_at,
            require_selected_pairing,
        )?;
        self.remember_accepted_intent_id(intent.intent_id.clone());
        Ok(())
    }

    pub(super) fn validate_intent_without_consuming(
        &self,
        intent: &LanParentIntentEnvelope,
        origin: Option<&str>,
        observed_at: &str,
        require_selected_pairing: bool,
    ) -> Result<(), LanPairingRejectionReason> {
        shape::validate_intent_shape(self, intent)?;
        let entry = entry::validate_intent_entry(self, intent, origin)?;
        selection::validate_selected_pairing(
            self,
            intent,
            entry,
            observed_at,
            require_selected_pairing,
        )?;

        Ok(())
    }
}
