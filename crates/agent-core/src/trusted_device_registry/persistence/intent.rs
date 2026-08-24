use std::{io, path::Path};

use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingRejectionReason, LanParentIntentEnvelope,
};

use super::super::TrustedDeviceRegistry;

impl TrustedDeviceRegistry {
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
        let intent = intent.clone();
        let origin = origin.map(str::to_owned);
        let observed_at = observed_at.to_owned();
        self.mutate_persisted_registry(registry_path, move |candidate| {
            Ok(candidate.validate_intent(&intent, origin.as_deref(), observed_at.as_str()))
        })
    }
}
