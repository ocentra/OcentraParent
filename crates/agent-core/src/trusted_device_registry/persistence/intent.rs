use std::{io, path::Path};

use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingRejectionReason, LanParentIntentEnvelope,
};

use super::super::TrustedDeviceRegistry;

impl TrustedDeviceRegistry {
    pub fn apply_intent<T>(
        &mut self,
        intent: &LanParentIntentEnvelope,
        origin: Option<&str>,
        observed_at: &str,
        require_selected_pairing: bool,
        effect: impl FnOnce(&mut Self) -> Result<T, LanPairingRejectionReason>,
    ) -> Result<T, LanPairingRejectionReason> {
        let mut candidate = self.clone();
        candidate.validate_intent_without_consuming(
            intent,
            origin,
            observed_at,
            require_selected_pairing,
        )?;
        let result = effect(&mut candidate)?;
        candidate.remember_accepted_intent_id(intent.intent_id.clone());
        *self = candidate;
        Ok(result)
    }

    pub fn apply_intent_persisted<T>(
        &mut self,
        registry_path: &Path,
        intent: &LanParentIntentEnvelope,
        origin: Option<&str>,
        observed_at: &str,
        require_selected_pairing: bool,
        effect: impl FnOnce(&mut Self) -> Result<T, LanPairingRejectionReason>,
    ) -> io::Result<Result<T, LanPairingRejectionReason>> {
        let intent = intent.clone();
        let origin = origin.map(str::to_owned);
        let observed_at = observed_at.to_owned();
        self.mutate_persisted_registry(registry_path, move |candidate| {
            Ok(candidate.apply_intent(
                &intent,
                origin.as_deref(),
                observed_at.as_str(),
                require_selected_pairing,
                effect,
            ))
        })
    }

    /// Validate and consume a control intent while holding the same durable
    /// registry lock used for every other persisted mutation.
    ///
    /// The in-memory validation path is still useful for explicitly ephemeral
    /// runtimes, but a local-json runtime must not acknowledge an intent until
    /// its id is part of the atomically written registry state.
    pub fn validate_intent_persisted(
        &mut self,
        registry_path: &Path,
        intent: &LanParentIntentEnvelope,
        origin: Option<&str>,
        observed_at: &str,
    ) -> io::Result<Result<(), LanPairingRejectionReason>> {
        self.validate_intent_persisted_with_selection_requirement(
            registry_path,
            intent,
            origin,
            observed_at,
            true,
        )
    }

    /// Validate and durably consume an intent whose operation selects its
    /// pairing explicitly rather than requiring a previously selected route.
    pub fn validate_selection_intent_persisted(
        &mut self,
        registry_path: &Path,
        intent: &LanParentIntentEnvelope,
        origin: Option<&str>,
        observed_at: &str,
    ) -> io::Result<Result<(), LanPairingRejectionReason>> {
        self.validate_intent_persisted_with_selection_requirement(
            registry_path,
            intent,
            origin,
            observed_at,
            false,
        )
    }

    fn validate_intent_persisted_with_selection_requirement(
        &mut self,
        registry_path: &Path,
        intent: &LanParentIntentEnvelope,
        origin: Option<&str>,
        observed_at: &str,
        require_selected_pairing: bool,
    ) -> io::Result<Result<(), LanPairingRejectionReason>> {
        self.apply_intent_persisted(
            registry_path,
            intent,
            origin,
            observed_at,
            require_selected_pairing,
            |_registry| Ok(()),
        )
    }
}
